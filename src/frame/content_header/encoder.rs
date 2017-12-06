use bytes::{BigEndian, BufMut};

use super::{ContentHeaderPayload, Properties};
use args::{AmqpString, FieldArgument};

use std::collections::HashMap;

const MIN_PAYLOAD_SIZE: usize = 14;

pub fn encode_payload(payload: ContentHeaderPayload) -> Vec<u8> {
    debug!("Start encoging conent header");

    let mut dst = Vec::with_capacity(MIN_PAYLOAD_SIZE);

    dst.put_u16::<BigEndian>(payload.class_id);
    debug!("class : {}", payload.class_id);

    // "weitht" field must be 0.
    dst.put_u16::<BigEndian>(0);

    dst.put_u64::<BigEndian>(payload.body_size);
    debug!("body size : {}", payload.body_size);

    // Encode property flags
    let property_flags = property_flags(&payload.properties);
    dst.put_u16::<BigEndian>(property_flags);
    debug!("property flags : {}", property_flags);

    // Encode property list
    if property_flags != 0b_0000_0000_0000_0001 {
        property_list(&payload.properties, &mut dst);
    }

    dst
}


fn property_flags(ps: &Properties) -> u16 {
    0b_____1000000000000000 * ps.content_type.is_some() as u16 +
        0b_0100000000000000 * ps.content_encoding.is_some() as u16 +
        0b_0010000000000000 * ps.headers.is_some() as u16 +
        0b_0001000000000000 * ps.delivery_mode.is_some() as u16 +
        0b_0000100000000000 * ps.priority.is_some() as u16 +
        0b_0000010000000000 * ps.correlation_id.is_some() as u16 +
        0b_0000001000000000 * ps.reply_to.is_some() as u16 +
        0b_0000000100000000 * ps.expiration.is_some() as u16 +
        0b_0000000010000000 * ps.message_id.is_some() as u16 +
        0b_0000000001000000 * ps.timestamp.is_some() as u16 +
        0b_0000000000100000 * ps.type_.is_some() as u16 +
        0b_0000000000010000 * ps.user_id.is_some() as u16 +
        0b_0000000000001000 * ps.app_id.is_some() as u16 +

        // No further properties.
        0b_0000_0000_0000_0001
}


fn property_list(ps: &Properties, dst: &mut Vec<u8>) {
    if let Some(ref s) = ps.content_type {
        encode_short_str(s, dst);
    }

    if let Some(ref s) = ps.content_encoding {
        encode_short_str(s, dst);
    }

    if let Some(ref f) = ps.headers {
        encode_field_table(f, dst);
    }

    if let Some(ref o) = ps.delivery_mode {
        dst.put_u8(*o);
    }

    if let Some(ref o) = ps.priority {
        dst.put_u8(*o);
    }

    if let Some(ref s) = ps.correlation_id {
        encode_short_str(s, dst);
    }

    if let Some(ref s) = ps.reply_to {
        encode_short_str(s, dst);
    }

    if let Some(ref s) = ps.expiration {
        encode_short_str(s, dst);
    }

    if let Some(ref s) = ps.message_id {
        encode_short_str(s, dst);
    }

    if let Some(ref t) = ps.timestamp {
        dst.put_i64::<BigEndian>(*t);
    }

    if let Some(ref s) = ps.type_ {
        encode_short_str(s, dst);
    }

    if let Some(ref s) = ps.user_id {
        encode_short_str(s, dst);
    }

    if let Some(ref s) = ps.app_id {
        encode_short_str(s, dst);
    }
}


fn encode_short_str(s: &AmqpString, dst: &mut Vec<u8>) {
    dst.put_u8(s.len() as u8);
    dst.put(s.as_bytes());
}

fn encode_field_table(f: &HashMap<AmqpString, FieldArgument>, dst: &mut Vec<u8>) {
    ::frame::method::encoder::encode_field_table_0(f, dst);
}
