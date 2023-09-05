use crate::{MessageBody, RawParts};
use byteorder::ByteOrder;
use byteorder::NetworkEndian;
use bytes::{Buf, BufMut, BytesMut};
use common_message::MessageId;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::ops::Range;
use tokio_util::codec::{Decoder, Encoder};

pub struct LenProtoMidCodec;

impl LenProtoMidCodec {
    pub const HEADER_LEN: usize = 8;
    pub const MAX_BODY_LEN: usize = 8000;
    pub const HEADER_LOCATION_PAYLOAD_SIZE: Range<usize> = 0..4;
    pub const HEADER_LOCATION_PROTOCOL_VERSION: Range<usize> = 4..6;
    pub const HEADER_LOCATION_MESSAGE_ID: Range<usize> = 6..8;

    /// Might panic if the given header is not at
    /// least `LenProtoMidCodec::HEADER_LEN` long
    fn read_payload_size(message_header: &[u8]) -> usize {
        NetworkEndian::read_u32(&message_header[Self::HEADER_LOCATION_PAYLOAD_SIZE]) as usize
    }

    /// Might panic if the given header is not at
    /// least `LenProtoMidCodec::HEADER_LEN` long
    fn read_protocol_version(message_header: &[u8]) -> u16 {
        NetworkEndian::read_u16(&message_header[Self::HEADER_LOCATION_PROTOCOL_VERSION])
    }

    /// Might panic if the given header is not at
    /// least `LenProtoMidCodec::HEADER_LEN` long
    fn read_message_id(message_header: &[u8]) -> u16 {
        NetworkEndian::read_u16(&message_header[Self::HEADER_LOCATION_MESSAGE_ID])
    }
}

impl Encoder<&RawParts> for LenProtoMidCodec {
    type Error = IoError;

    fn encode(&mut self, item: &RawParts, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u32(item.body.len() as u32);
        dst.put_u16(item.protocol);
        dst.put_u16(item.message_id.into());
        dst.put_slice(&item.body[..]);
        Ok(())
    }
}

impl Decoder for LenProtoMidCodec {
    type Item = RawParts;
    type Error = IoError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < Self::HEADER_LEN {
            Ok(None)
        } else {
            let payload_size = Self::read_payload_size(&src[..]);
            let protocol_version = Self::read_protocol_version(&src[..]);
            let message_id = Self::read_message_id(&src[..]);

            if payload_size > Self::MAX_BODY_LEN {
                Err(IoError::new(
                    IoErrorKind::Other,
                    format!(
                        "Max body length exceeded: {}/{}",
                        payload_size,
                        Self::MAX_BODY_LEN
                    ),
                ))
            } else if src.len() < Self::HEADER_LEN + payload_size {
                Ok(None)
            } else {
                src.advance(Self::HEADER_LEN); // header no longer required
                let body = src.split_to(payload_size);
                Ok(Some(RawParts {
                    protocol: protocol_version,
                    message_id: MessageId::from(message_id),
                    body: MessageBody::Bytes(body.freeze()),
                }))
            }
        }
    }
}
