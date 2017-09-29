use bytes::{Bytes, BigEndian, Buf};

use std::io::Cursor;

use super::ContentHeaderPayload;
use errors::Error;

pub fn decode_payload(payload: &mut Cursor<Bytes>) -> Result<ContentHeaderPayload, Error> {
    let class_id = payload.get_u16::<BigEndian>();
    let weight = payload.get_u16::<BigEndian>(); // must be zero
    assert_eq!(weight, 0);
    let body_size = payload.get_u64::<BigEndian>();
    let property_flags = payload.get_u16::<BigEndian>();
    debug!("remainder num {}",
           payload.get_ref().len() - payload.position() as usize);

    let payload = ContentHeaderPayload {
        class_id: class_id,
        body_size: body_size,
        property_flags: property_flags,
    };

    Ok(payload)
}
