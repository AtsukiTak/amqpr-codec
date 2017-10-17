pub mod decoder;
pub mod encoder;

pub mod method;
pub mod content_body;
pub mod content_header;


use self::method::MethodPayload;
use self::content_header::ContentHeaderPayload;
use self::content_body::ContentBodyPayload;


pub const FRAME_END_OCTET: u8 = 0xCE;


#[derive(PartialEq, Clone, Debug)]
pub struct Frame {
    pub header: FrameHeader,
    pub payload: FramePayload,
}


#[derive(PartialEq, Clone, Debug)]
pub struct FrameHeader {
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


// Implementation of Frame {{{
impl Frame {
    pub fn new(channel: u16, payload: FramePayload) -> Frame {
        Frame {
            header: FrameHeader { channel: channel },
            payload: payload,
        }
    }

    pub fn new_method(channel: u16, payload: MethodPayload) -> Frame {
        Frame::new(channel, FramePayload::Method(payload))
    }

    pub fn new_content_header(channel: u16, payload: ContentHeaderPayload) -> Frame {
        Frame::new(channel, FramePayload::ContentHeader(payload))
    }

    pub fn new_content_body(channel: u16, payload: ContentBodyPayload) -> Frame {
        Frame::new(channel, FramePayload::ContentBody(payload))
    }

    pub fn new_heartbeat(channel: u16) -> Frame {
        Frame::new(channel, FramePayload::Heartbeat)
    }


    pub fn method(&self) -> Option<&MethodPayload> {
        match &self.payload {
            &FramePayload::Method(ref p) => Some(p),
            _ => None,
        }
    }

    pub fn content_header(&self) -> Option<&ContentHeaderPayload> {
        match &self.payload {
            &FramePayload::ContentHeader(ref p) => Some(p),
            _ => None,
        }
    }

    pub fn content_body(&self) -> Option<&ContentBodyPayload> {
        match &self.payload {
            &FramePayload::ContentBody(ref p) => Some(p),
            _ => None,
        }
    }

    pub fn heartbeat(&self) -> Option<()> {
        match &self.payload {
            &FramePayload::Heartbeat => Some(()),
            _ => None,
        }
    }
}
// }}}
