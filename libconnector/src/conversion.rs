use crate::pods::message::{CrMessageId, CrMessageIdOverloaded};
use crate::result::CrResult;
use asn1rs::prelude::*;
use common_edge::asn1rs;
use common_edge::common_message_codec;
use common_message_codec::bson;
use common_message_codec::EdgeMessage;
use log::error;
use std::os::raw::c_char;

#[derive(thiserror::Error, Debug)]
pub(crate) enum DecodeError {
    #[error("Uper decoding error: {0}")]
    Uper(#[from] asn1rs::io::per::Error),
    #[error("Json decoding error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Bson decoding error: {0}")]
    Bson(#[from] bson::de::Error),
    #[error("Protobuf decoding error: {0}")]
    Protobuf(#[from] asn1rs::io::protobuf::Error),
    #[error("Utf8 decoding error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}
#[derive(thiserror::Error, Debug)]
pub(crate) enum EncodeError {
    #[error("Uper encoding error: {0}")]
    Uper(#[from] asn1rs::io::per::Error),
    #[error("Json encoding error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Bson encoding error: {0}")]
    Bson(#[from] bson::ser::Error),
    #[error("Protobuf encoding error: {0}")]
    Protobuf(#[from] asn1rs::io::protobuf::Error),
    #[error("Utf8 encoding error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
pub enum CrFormat {
    Uper = 0,
    Json,
    JsonPretty,
    Bson,
    Protobuf,
}

impl CrFormat {
    pub(crate) fn decode(self, mid: CrMessageId, src: &[u8]) -> Result<EdgeMessage, DecodeError> {
        macro_rules! repeat {
            ($mid:expr, [$($name:ident,)+], $code:expr) => {
                match $mid {
                    $(CrMessageId::$name => EdgeMessage::$name($code),)+
                }
            };
            ($mid:expr, $code:expr) => {
                repeat!(
                    $mid,
                    [
                        Denm,
                        Cpm,
                        Cam,
                        Vam,
                        Mcm,
                        ApplicationInfo,
                        DebugRequest,
                        ComponentStatus,
                    ],
                    $code
                )
            };
        }

        match self {
            CrFormat::Uper => {
                let mut reader = UperReader::from(src);
                Ok(match mid {
                    CrMessageId::Denm => EdgeMessage::Denm(reader.read()?),
                    CrMessageId::Cpm => EdgeMessage::Cpm(reader.read()?),
                    CrMessageId::Cam => EdgeMessage::Cam(reader.read()?),
                    CrMessageId::Vam => EdgeMessage::Vam(reader.read()?),
                    CrMessageId::Mcm => EdgeMessage::Mcm(reader.read()?),
                    CrMessageId::ApplicationInfo => EdgeMessage::ApplicationInfo(reader.read()?),
                    CrMessageId::DebugRequest => EdgeMessage::DebugRequest(reader.read()?),
                    CrMessageId::ComponentStatus => EdgeMessage::ComponentStatus(reader.read()?),
                })
            }
            CrFormat::Json | CrFormat::JsonPretty => {
                let string = String::from_utf8(src.to_vec())?;
                Ok(repeat!(
                    mid,
                    serde_json::from_reader(&mut string.as_bytes())?
                ))
            }
            CrFormat::Bson => Ok(repeat!(mid, bson::from_reader(&mut &*src)?)),
            CrFormat::Protobuf => {
                let mut reader = ProtobufReader::from(src);
                Ok(match mid {
                    CrMessageId::Denm => EdgeMessage::Denm(reader.read()?),
                    CrMessageId::Cpm => EdgeMessage::Cpm(reader.read()?),
                    CrMessageId::Cam => EdgeMessage::Cam(reader.read()?),
                    CrMessageId::Vam => EdgeMessage::Vam(reader.read()?),
                    CrMessageId::Mcm => EdgeMessage::Mcm(reader.read()?),
                    CrMessageId::ApplicationInfo => EdgeMessage::ApplicationInfo(reader.read()?),
                    CrMessageId::DebugRequest => EdgeMessage::DebugRequest(reader.read()?),
                    CrMessageId::ComponentStatus => EdgeMessage::ComponentStatus(reader.read()?),
                })
            }
        }
    }

    pub(crate) fn encode(self, msg: &EdgeMessage) -> Result<Vec<u8>, EncodeError> {
        match self {
            CrFormat::Uper => Ok(msg.serialize_inner_to_uper_bytes_vec()?.0),
            CrFormat::Json => Ok(msg.serialize_inner_to_json_bytes_vec()?),
            CrFormat::JsonPretty => Ok(msg.serialize_inner_to_json_pretty_bytes_vec()?),
            CrFormat::Bson => Ok(msg.serialize_inner_to_bson_bytes_vec()?),
            CrFormat::Protobuf => Ok(msg.serialize_inner_to_protobuf_bytes_vec()?),
        }
    }
}

/// Decodes the message associated for the given [`CrMessageId] from the source
/// buffer with the specified format. Re-encodes it into the given target buffer
/// with the desired output format. The `dst_len` value is updated to reflect the
/// to the target buffer written bytes.
///
/// Please be aware that messages of type `UulmRawBson` cannot be converted into
/// `UPER` or `Protobuf` blobs due to missing schema definition.  
///
/// # Safety
///
/// The source and target buffer must point to a valid memory region of at least the
/// given size.
#[no_mangle]
pub unsafe extern "C" fn cr_message_convert(
    mid: CrMessageId,
    src_type: CrFormat,
    src: *const c_char,
    src_len: usize,
    dst_type: CrFormat,
    dst: *mut c_char,
    dst_len: *mut usize,
) -> CrResult {
    let mid = match CrMessageIdOverloaded::from(mid).into_single() {
        Some(mid) => mid,
        None => return CrResult::ErrParameterMessageIdIsInvalid,
    };

    let source = if src.is_null() {
        return CrResult::ErrParameterSourceBufferIsNull;
    } else {
        core::slice::from_raw_parts(src as *const u8, src_len)
    };

    let dst_len = if dst_len.is_null() {
        return CrResult::ErrParameterTargetBufferLenIsNull;
    } else {
        &mut *dst_len
    };

    let target = if dst.is_null() {
        return CrResult::ErrParameterTargetBufferIsNull;
    } else {
        core::slice::from_raw_parts_mut(dst as *mut u8, *dst_len)
    };

    let message = match src_type.decode(mid, source) {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("{e}");
            error!("{e}");
            return match src_type {
                CrFormat::Uper => CrResult::ErrUperDecodingFailed,
                CrFormat::Json | CrFormat::JsonPretty => CrResult::ErrJsonDecodingFailed,
                CrFormat::Bson => CrResult::ErrBsonDecodingFailed,
                CrFormat::Protobuf => CrResult::ErrProtobufDecodingFailed,
            };
        }
    };

    match dst_type.encode(&message) {
        Ok(vec) => {
            if vec.len() <= target.len() {
                target[..vec.len()].copy_from_slice(&vec[..]);
                *dst_len = vec.len();
                CrResult::Ok
            } else {
                CrResult::ErrParameterTargetBufferInsufficientSize
            }
        }
        Err(e) => {
            eprintln!("{e}");
            error!("{e}");
            match dst_type {
                CrFormat::Uper => CrResult::ErrUperDecodingFailed,
                CrFormat::Json | CrFormat::JsonPretty => CrResult::ErrJsonDecodingFailed,
                CrFormat::Bson => CrResult::ErrBsonDecodingFailed,
                CrFormat::Protobuf => CrResult::ErrProtobufDecodingFailed,
            }
        }
    }
}
