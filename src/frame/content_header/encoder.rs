use bytes::{BigEndian, BufMut};

use std::io::Cursor;

use super::ContentHeaderPayload;

pub fn encode_payload(payload: ContentHeaderPayload) -> [u8; 14] {
    debug!("Start endoging conent header");

    let mut dst = Cursor::new([0; 14]);

    dst.put_u16::<BigEndian>(payload.class_id);
    debug!("class : {}", payload.class_id);

    // "weitht" field must be 0.
    dst.put_u16::<BigEndian>(0);

    dst.put_u64::<BigEndian>(payload.body_size);
    debug!("body size : {}", payload.body_size);

    dst.put_u16::<BigEndian>(payload.property_flags);
    debug!("property_flags : {}", payload.property_flags);

    dst.into_inner()
}
