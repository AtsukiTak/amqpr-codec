pub mod decoder;
pub mod encoder;

pub use self::decoder::decode_payload;
pub use self::encoder::encode_payload;


#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ContentHeaderPayload {
    pub class_id: u16,
    pub body_size: u64,
    pub property_flags: u16,
}
