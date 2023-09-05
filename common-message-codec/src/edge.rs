use crate::CodecError;
use crate::LenProtoMidCodec;
use crate::ProtocolVersion;
use crate::{MessageBody, RawParts, StaticDecoder, StaticEncoder};
use asn1rs::prelude::*;
use bytes::BytesMut;
use common_message::MessageId;
use messages::cam_pdu_descriptions::Cam;
use messages::cpm_pdu_descriptions::Cpm;
use messages::itd_data_protocol::ApplicationInfo;
use messages::denm_pdu_descriptions::Denm;
use messages::itd_ssdm_descriptions::{ComponentStatus, DebugRequest};
use messages::mcm_pdu_descriptions::Mcm;
use messages::vam_pdu_descriptions::Vam;
use tokio_util::codec::{Decoder, Encoder};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum EdgeMessage {
    Denm(Denm),
    Cpm(Cpm),
    Cam(Cam),
    Vam(Vam),
    Mcm(Mcm),

    DebugRequest(DebugRequest),
    ComponentStatus(ComponentStatus),

    ApplicationInfo(ApplicationInfo),
}
macro_rules! serialize_inner {
    ($msg:expr, ($v:ident) => $serializer:expr) => {{
        serialize_inner!($msg, ($v) => $serializer, ($v) => $serializer)
    }};
    ($msg:expr, ($v:ident) => $serializer:expr, ($b:ident) => $bson_serializer:expr) => {{
        match $msg {
            EdgeMessage::Denm($v) =>  $serializer,
            EdgeMessage::Cpm($v) => $serializer,
            EdgeMessage::Cam($v) => $serializer,
            EdgeMessage::Vam($v) => $serializer,
            EdgeMessage::Mcm($v) => $serializer,
            EdgeMessage::DebugRequest($v) => $serializer,
            EdgeMessage::ComponentStatus($v) => $serializer,
            EdgeMessage::ApplicationInfo($v) => $serializer,
        }
    }}
}

impl EdgeMessage {
    pub const fn uper_codec() -> EdgeMessageUperCodec {
        EdgeMessageUperCodec
    }

    pub const fn id(&self) -> MessageId {
        match self {
            EdgeMessage::Denm(_) => MessageId::Denm,
            EdgeMessage::Cpm(_) => MessageId::Cpm,
            EdgeMessage::Cam(_) => MessageId::Cam,
            EdgeMessage::Vam(_) => MessageId::Vam,
            EdgeMessage::Mcm(_) => MessageId::Mcm,
            EdgeMessage::DebugRequest(_) => MessageId::DebugRequest,
            EdgeMessage::ComponentStatus(_) => MessageId::ComponentStatus,
            EdgeMessage::ApplicationInfo(_) => MessageId::ApplicationInfo,
        }
    }

    pub fn inner_to_json_writer_pretty(
        &self,
        writer: impl std::io::Write,
    ) -> serde_json::Result<()> {
        serialize_inner!(self, (v) => serde_json::to_writer_pretty(writer, v))
    }

    pub fn serialize_inner_to_json_bytes_vec(&self) -> serde_json::Result<Vec<u8>> {
        serialize_inner!(self, (v) => serde_json::to_vec(v))
    }

    pub fn serialize_inner_to_json_pretty_bytes_vec(&self) -> serde_json::Result<Vec<u8>> {
        serialize_inner!(self, (v) => serde_json::to_vec_pretty(v))
    }

    pub fn serialize_inner_to_bson_bytes_vec(&self) -> bson::ser::Result<Vec<u8>> {
        serialize_inner!(self, (v) => bson::to_vec(v))
    }

    pub fn serialize_inner_to_protobuf_bytes_vec(
        &self,
    ) -> Result<Vec<u8>, asn1rs::io::protobuf::Error> {
        serialize_inner!(self, (v) => {
            crate::protobuf::to_vec(v)
        }, (_b) => {
            Err(asn1rs::io::protobuf::Error::invalid_format(0))
        })
    }

    pub fn serialize_inner_to_uper_bytes_vec(
        &self,
    ) -> Result<(Vec<u8>, usize), asn1rs::io::per::Error> {
        serialize_inner!(self, (v) => {
            crate::uper::to_vec(v)
        }, (_b) => {
            Err(asn1rs::io::per::ErrorKind::UnsupportedOperation("Cannot serialize BSON to uPER without ASN.1 definition".to_string()).into())
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EdgeMessageUperCodec;

#[allow(unused)]
impl EdgeMessageUperCodec {
    pub const PROTOCOL_LUKAS_V1: ProtocolVersion = crate::protocol::LUKAS_V1;
    pub const PROTOCOL_LUKAS_V2: ProtocolVersion = crate::protocol::LUKAS_V2;
    pub const PROTOCOL_LUKAS_V3: ProtocolVersion = crate::protocol::LUKAS_V3;
    pub const PROTOCOL_LUKAS_V4: ProtocolVersion = crate::protocol::LUKAS_V4;
    pub const PROTOCOL_LUKAS_V5: ProtocolVersion = crate::protocol::LUKAS_V5;
    pub const PROTOCOL_LUKAS_V6: ProtocolVersion = crate::protocol::LUKAS_V6;
    pub const PROTOCOL_LUKAS_CURRENT: ProtocolVersion = crate::protocol::LUKAS_CURRENT;
    pub const MAX_BODY_LENGTH: usize = 0x7F_FF; // ~32kib (-1byte)
}

impl Encoder<&RawParts> for EdgeMessageUperCodec {
    type Error = CodecError;

    fn encode(&mut self, item: &RawParts, dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(LenProtoMidCodec.encode(item, dst)?)
    }
}

impl StaticEncoder for EdgeMessageUperCodec {
    type Target = RawParts;
    type Item = EdgeMessage;
    type Error = CodecError;

    fn guess_meta(item: &Self::Item) -> Self::Target {
        RawParts {
            protocol: Self::PROTOCOL_LUKAS_CURRENT,
            message_id: item.id(),
            body: MessageBody::Vec(Vec::new()),
        }
    }

    fn encode_into(item: &Self::Item) -> Result<Self::Target, Self::Error> {
        let mut parts = Self::guess_meta(item);
        let mut writer = UperWriter::with_capacity(Self::MAX_BODY_LENGTH);
        match item {
            EdgeMessage::Denm(m) => writer.write(m)?,
            EdgeMessage::Cpm(m) => writer.write(m)?,
            EdgeMessage::Cam(m) => writer.write(m)?,
            EdgeMessage::Vam(m) => writer.write(m)?,
            EdgeMessage::Mcm(m) => writer.write(m)?,
            EdgeMessage::DebugRequest(m) => writer.write(m)?,
            EdgeMessage::ComponentStatus(m) => writer.write(m)?,
            EdgeMessage::ApplicationInfo(m) => writer.write(m)?,
        }
        parts.body = MessageBody::Vec(writer.into_bytes_vec());
        Ok(parts)
    }
}

impl Decoder for EdgeMessageUperCodec {
    type Item = RawParts;
    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(LenProtoMidCodec.decode(src)?)
    }
}

impl StaticDecoder for EdgeMessageUperCodec {
    type Source = RawParts;
    type Item = EdgeMessage;
    type Error = CodecError;

    fn decode_from(source: &Self::Source) -> Result<Self::Item, Self::Error> {
        Self::Error::check_protocol_version(Self::PROTOCOL_LUKAS_CURRENT, source.protocol)?;
        let mut reader = UperReader::from(&source.body[..]);
        Ok(match source.message_id {
            MessageId::Denm => EdgeMessage::Denm(reader.read()?),
            MessageId::Cpm => EdgeMessage::Cpm(reader.read()?),
            MessageId::Cam => EdgeMessage::Cam(reader.read()?),
            MessageId::Vam => EdgeMessage::Vam(reader.read()?),
            MessageId::Mcm => EdgeMessage::Mcm(reader.read()?),
            MessageId::DebugRequest => EdgeMessage::DebugRequest(reader.read()?),
            MessageId::ComponentStatus => EdgeMessage::ComponentStatus(reader.read()?),
            MessageId::ApplicationInfo => EdgeMessage::ApplicationInfo(reader.read()?),
            mid => return Err(CodecError::unexpected_message_id(mid)),
        })
    }
}
