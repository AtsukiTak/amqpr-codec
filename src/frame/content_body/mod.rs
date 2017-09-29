pub mod decoder;
pub mod encoder;

pub use self::decoder::decode_payload;

use bytes::Bytes;


/// Content body payload is just binary payload
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ContentBodyPayload {
    pub bytes: Bytes,
}


impl ContentBodyPayload {
    pub fn byte_size(&self) -> u32 {
        // length of payload + frame-end
        (self.bytes.len() + 1) as u32
    }
}
