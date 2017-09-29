use bytes::{BytesMut, BigEndian, BufMut};

use super::ContentHeaderPayload;
use errors::*;

pub fn encode_payload(payload: ContentHeaderPayload, dst: &mut BytesMut) -> Result<()> {
    dst.reserve(14);

    dst.put_u16::<BigEndian>(payload.class_id);
    dst.put_u16::<BigEndian>(0);
    dst.put_u64::<BigEndian>(payload.body_size);
    dst.put_u16::<BigEndian>(payload.property_flags);

    Ok(())
}


pub fn byte_size_of_payload(payload: &ContentHeaderPayload) -> u32 {
    14_u32
}
