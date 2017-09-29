pub mod decoder;
pub mod encoder;

pub mod method;
pub mod content_body;
pub mod content_header;


use tokio_io::codec::{Decoder, Encoder};

use bytes::BytesMut;

use self::method::MethodPayload;
use self::content_header::ContentHeaderPayload;
use self::content_body::ContentBodyPayload;

use errors::*;


pub const FRAME_END_OCTET: u8 = 0xCE;


#[derive(PartialEq, Clone, Debug)]
pub struct Frame {
    pub header: FrameHeader,
    pub payload: FramePayload,
}


#[derive(PartialEq, Clone, Debug)]
pub struct FrameHeader {
    pub frame_type: FrameType,
    pub channel: u16,
}


#[derive(Eq, PartialEq, Clone, Debug)]
pub enum FrameType {
    Method,
    ContentHeader,
    ContentBody,
    Heartbeat,
}


#[derive(PartialEq, Clone, Debug)]
pub enum FramePayload {
    Method(MethodPayload),
    ContentHeader(ContentHeaderPayload),
    ContentBody(ContentBodyPayload),
    Heartbeat,
}

pub struct Codec;

impl Decoder for Codec {
    type Item = Frame;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        self::decoder::decode_frame(src)
    }
}

impl Encoder for Codec {
    type Item = Frame;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<()> {
        let mut buf = Vec::new();
        self::encoder::encode_frame(item, &mut buf)?;
        Ok(dst.extend_from_slice(buf.as_ref()))
    }
}
