use bytes::{BytesMut, BigEndian, Buf};

use std::io::Cursor;
use std::collections::HashMap;

use super::{ContentHeaderPayload, Properties};
use args::{AmqpString, FieldArgument};

const NUM_OF_PROPERTY: usize = 13;

pub fn decode_payload(payload: &mut BytesMut) -> ContentHeaderPayload {
    let properties = decode_properties(payload.split_off(12));

    let mut others_cursor = Cursor::new(payload.take());

    let class_id = others_cursor.get_u16::<BigEndian>();
    assert!(class_id != 0);

    let weight = others_cursor.get_u16::<BigEndian>(); // must be zero
    assert_eq!(weight, 0);

    let body_size = others_cursor.get_u64::<BigEndian>();
    drop(others_cursor);

    let payload = ContentHeaderPayload {
        class_id: class_id,
        body_size: body_size,
        properties: properties,
    };

    payload
}



fn decode_properties(mut bytes: BytesMut) -> Properties {
    let mut flags = Cursor::new(bytes.split_to(2)).get_u16::<BigEndian>();

    let mut ps = Properties::new();

    for i in 0..NUM_OF_PROPERTY {
        if flags == 0 {
            break;
        }
        if check_flag_n(&flags, i) {
            remove_flag_n(&mut flags, i);
            set_property_n(&mut ps, i, &mut bytes);
        }
    }

    ps
}


fn check_flag_n(flags: &u16, i: usize) -> bool {
    flags & (1u16 << (15 - i)) != 0
}


fn remove_flag_n(flags: &mut u16, i: usize) {
    *flags -= 1u16 << (15 - i);
}

fn set_property_n(ps: &mut Properties, i: usize, bytes: &mut BytesMut) {
    match i {
        0 => ps.content_type = Some(decode_short_str(bytes)),
        1 => ps.content_encoding = Some(decode_short_str(bytes)),
        2 => ps.headers = Some(decode_field_table(bytes)),
        3 => ps.delivery_mode = Some(decode_u8(bytes)),
        4 => ps.priority = Some(decode_u8(bytes)),
        5 => ps.correlation_id = Some(decode_short_str(bytes)),
        6 => ps.reply_to = Some(decode_short_str(bytes)),
        7 => ps.expiration = Some(decode_short_str(bytes)),
        8 => ps.message_id = Some(decode_short_str(bytes)),
        9 => ps.timestamp = Some(decode_i64(bytes)),
        10 => ps.type_ = Some(decode_short_str(bytes)),
        11 => ps.user_id = Some(decode_short_str(bytes)),
        12 => ps.app_id = Some(decode_short_str(bytes)),
        _ => unreachable!(),
    }
}


fn decode_short_str(bytes: &mut BytesMut) -> AmqpString {
    let len = Cursor::new(bytes.split_to(1)).get_u8();
    AmqpString(bytes.split_to(len as usize).freeze())
}


fn decode_u8(bytes: &mut BytesMut) -> u8 {
    Cursor::new(bytes.split_to(1)).get_u8()
}


fn decode_i64(bytes: &mut BytesMut) -> i64 {
    Cursor::new(bytes.split_to(8)).get_i64::<BigEndian>()
}


fn decode_field_table(bytes: &mut BytesMut) -> HashMap<AmqpString, FieldArgument> {
    ::frame::method::decoder::decode_field_table(bytes)
}
