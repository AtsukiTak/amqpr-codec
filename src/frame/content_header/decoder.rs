use bytes::{BytesMut, BigEndian, Buf};

use std::io::Cursor;

use super::ContentHeaderPayload;

pub fn decode_payload(payload: BytesMut) -> ContentHeaderPayload {
    let mut cursor = Cursor::new(payload);

    let class_id = cursor.get_u16::<BigEndian>();
    assert!(class_id != 0);

    let weight = cursor.get_u16::<BigEndian>(); // must be zero
    assert_eq!(weight, 0);

    let body_size = cursor.get_u64::<BigEndian>();

    let property_flags = cursor.get_u16::<BigEndian>();

    let payload = ContentHeaderPayload {
        class_id: class_id,
        body_size: body_size,
        property_flags: property_flags,
    };

    payload
}
