use bytes::{BigEndian, BufMut};

use super::ContentHeaderPayload;
use errors::*;

pub fn encode_payload(payload: ContentHeaderPayload, dst: &mut Vec<u8>) -> Result<()> {
    debug!("Start endoging conent header");
    // dst.reserve(14);

    dst.put_u16::<BigEndian>(payload.class_id);
    debug!("class : {}", payload.class_id);
    dst.put_u16::<BigEndian>(0);
    dst.put_u64::<BigEndian>(payload.body_size);
    debug!("body size : {}", payload.body_size);
    dst.put_u16::<BigEndian>(payload.property_flags);
    debug!("property_flags : {}", payload.property_flags);

    Ok(())
}


pub fn byte_size_of_payload(_payload: &ContentHeaderPayload) -> u32 {
    14_u32
}
