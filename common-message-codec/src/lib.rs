pub use bson;
pub use byteorder;
pub use bytes;
pub use serde_json;
pub use tokio_util;

pub type ProtocolVersion = u16;

#[allow(unused)]
pub mod protocol {
    use crate::ProtocolVersion;

    pub const MEC_VIEW_V5: ProtocolVersion = 5;
    pub const LUKAS_V1: ProtocolVersion = 6;
    pub const LUKAS_V2: ProtocolVersion = 7;
    pub const LUKAS_V3: ProtocolVersion = 8;
    pub const LUKAS_V4: ProtocolVersion = 9;
    pub const LUKAS_V5: ProtocolVersion = 10;
    pub const LUKAS_V6: ProtocolVersion = 11;
    pub const LUKAS_CURRENT: ProtocolVersion = LUKAS_V6;

    pub const EXT_OFFSET: ProtocolVersion = 0x00_80;
    pub const EXT_LUKAS_UULM_BSON_V1: ProtocolVersion = EXT_OFFSET;

    pub const BACKEND_OFFSET: ProtocolVersion = 0x00_C0;
    pub const BACKEND_LUKAS_EDGE_DATA: ProtocolVersion = BACKEND_OFFSET;
}

mod codec;
pub use codec::StaticDecoder;
pub use codec::StaticEncoder;

mod raw;
pub use raw::RawParts;

mod container;
pub use container::MessageBody;

mod lpmc;
pub use lpmc::*;

mod edge;
pub use edge::*;

mod err;
pub use err::CodecError;

pub mod protobuf {
    pub fn to_vec(
        value: &impl asn1rs::syn::Writable,
    ) -> Result<Vec<u8>, asn1rs::io::protobuf::Error> {
        let mut writer = asn1rs::syn::io::ProtobufWriter::default();
        value.write(&mut writer)?;
        Ok(writer.into_bytes_vec())
    }
}

pub mod uper {
    pub fn to_vec(
        value: &impl asn1rs::syn::Writable,
    ) -> Result<(Vec<u8>, usize), asn1rs::io::per::Error> {
        let mut writer = asn1rs::syn::io::UperWriter::default();
        value.write(&mut writer)?;
        let bit_len = writer.bit_len();
        Ok((writer.into_bytes_vec(), bit_len))
    }
}
