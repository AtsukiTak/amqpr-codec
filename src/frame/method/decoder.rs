use bytes::{Bytes, BytesMut, BigEndian, Buf};

use std::io::Cursor;
use std::collections::HashMap;

use frame::method::{MethodPayload, ConnectionClass, ChannelClass, ExchangeClass, QueueClass,
                    BasicClass, TxClass};
use args::*;

/// # NOTICE
/// This method does not check payload length.
pub fn decode_payload(payload: BytesMut) -> MethodPayload {
    let mut cursor = Cursor::new(payload.freeze());

    let class_id = cursor.get_u16::<BigEndian>();
    debug!("class_id is {}", class_id);

    let method_id = cursor.get_u16::<BigEndian>();
    debug!("method_id is {}", method_id);

    match class_id {
        10 => MethodPayload::Connection(decode_connection_class(method_id, &mut cursor)),
        20 => MethodPayload::Channel(decode_channel_class(method_id, &mut cursor)),
        40 => MethodPayload::Exchange(decode_exchange_class(method_id, &mut cursor)),
        50 => MethodPayload::Queue(decode_queue_class(method_id, &mut cursor)),
        60 => MethodPayload::Basic(decode_basic_class(method_id, &mut cursor)),
        90 => MethodPayload::Tx(decode_tx_class(method_id, &mut cursor)),
        c => unreachable!("Unexpected class id {}", c),
    }
}


// Decode Connection Class {{{
fn decode_connection_class(method_id: u16, payload: &mut Cursor<Bytes>) -> ConnectionClass {
    use frame::method::connection::*;
    use self::ConnectionClass::*;
    match method_id {
        10 => Start(StartMethod {
            version_major: decode_octet(payload),
            version_minor: decode_octet(payload),
            server_properties: decode_field_table(payload),
            mechanisms: decode_long_str(payload),
            locales: decode_long_str(payload),
        }),
        20 => Secure(SecureMethod { challenge: decode_long_str(payload) }),
        30 => Tune(TuneMethod {
            channel_max: decode_short(payload),
            frame_max: decode_long(payload),
            heartbeat: decode_short(payload),
        }),
        41 => OpenOk(OpenOkMethod { reserved1: decode_short_str(payload) }),
        50 => Close(CloseMethod {
            reply_code: decode_short(payload),
            reply_text: decode_short_str(payload),
            class_id: decode_short(payload),
            method_id: decode_short(payload),
        }),
        51 => CloseOk,
        60 => Blocked(BlockedMethod { reason: decode_short_str(payload) }),
        61 => Unblocked,
        m => unreachable!("Unexpected method id {} in Connection class", m),
    }
}
// }}}


// Decode Channel Class {{{
fn decode_channel_class(method_id: u16, payload: &mut Cursor<Bytes>) -> ChannelClass {
    use frame::method::channel::*;
    use self::ChannelClass::*;
    match method_id {
        11 => OpenOk(OpenOkMethod { reserved1: decode_long_str(payload) }),
        20 => Flow(FlowMethod { active: decode_bool_1(payload) }),
        21 => FlowOk(FlowOkMethod { active: decode_bool_1(payload) }),
        40 => Close(CloseMethod {
            reply_code: decode_short(payload),
            reply_text: decode_short_str(payload),
            class_id: decode_short(payload),
            method_id: decode_short(payload),
        }),
        41 => CloseOk,
        m => unreachable!("Unexpected method id {} in Channel class", m),
    }
}
// }}}


// Decode Exchange Class {{{
fn decode_exchange_class(method_id: u16, _payload: &mut Cursor<Bytes>) -> ExchangeClass {
    use self::ExchangeClass::*;
    match method_id {
        11 => DeclareOk,
        21 => DeleteOk,
        31 => BindOk, // rabbitmq-specific extension
        51 => UnbindOk, // rabbitmq-specific extension
        m => unreachable!("Unexpected method id {} in Channel class", m),
    }
}
// }}}


// Decode Queue Class {{{
fn decode_queue_class(method_id: u16, payload: &mut Cursor<Bytes>) -> QueueClass {
    use frame::method::queue::*;
    use self::QueueClass::*;
    match method_id {
        11 => DeclareOk(DeclareOkMethod {
            queue: decode_short_str(payload),
            message_count: decode_long(payload),
            consumer_count: decode_long(payload),
        }),
        21 => BindOk,
        31 => PurgeOk(PurgeOkMethod { message_count: decode_long(payload) }),
        41 => DeleteOk(DeleteOkMethod { message_count: decode_long(payload) }),
        51 => UnbindOk,
        m => unreachable!("Unexpected method id {} in Channel class", m),
    }
}
// }}}


// Decode Basic Class {{{
fn decode_basic_class(method_id: u16, payload: &mut Cursor<Bytes>) -> BasicClass {
    use frame::method::basic::*;
    use self::BasicClass::*;
    match method_id {
        11 => QosOk,
        21 => ConsumeOk(ConsumeOkMethod { consumer_tag: decode_short_str(payload) }),
        31 => CancelOk(CancelOkMethod { consumer_tag: decode_short_str(payload) }),
        50 => Return(ReturnMethod {
            reply_code: decode_short(payload),
            reply_text: decode_short_str(payload),
            exchange: decode_short_str(payload),
            routing_key: decode_short_str(payload),
        }),
        60 => Deliver(DeliverMethod {
            consumer_tag: decode_short_str(payload),
            delivery_tag: decode_longlong(payload),
            redeliverd: decode_bool_1(payload),
            exchange: decode_short_str(payload),
            routing_key: decode_short_str(payload),
        }),
        71 => GetOk(GetOkMethod {
            delivery_tag: decode_longlong(payload),
            redeliverd: decode_bool_1(payload),
            exchange: decode_short_str(payload),
            routing_key: decode_short_str(payload),
            message_count: decode_long(payload),
        }),
        72 => GetEmpty(GetEmptyMethod { reserved1: decode_short_str(payload) }),
        80 => Ack(AckMethod {
            delivery_tag: decode_longlong(payload),
            multiple: decode_bool_1(payload),
        }),

        // rabbitmq-specific extension
        120 => Nack(NackMethod {
            delivery_tag: decode_longlong(payload),
            multiple: decode_bool_1(payload),
        }),

        m => unreachable!("Unexpected method id {} in Channel class", m),
    }
}
// }}}


// Decode Tx Class {{{
fn decode_tx_class(method_id: u16, _payload: &mut Cursor<Bytes>) -> TxClass {
    use self::TxClass::*;
    match method_id {
        11 => SelectOk,
        21 => CommitOk,
        31 => RollbackOk,
        m => unreachable!("Unexpected method id {} in Connection class", m),
    }
}
// }}}


// Decode methods {{{
fn decode_bool_1(payload: &mut Cursor<Bytes>) -> bool {
    payload.get_u8() & 0b_1000_0000 == 0b_1000_0000
}


fn decode_octet(payload: &mut Cursor<Bytes>) -> u8 {
    payload.get_u8()
}


fn decode_short(payload: &mut Cursor<Bytes>) -> u16 {
    payload.get_u16::<BigEndian>()
}


fn decode_long(payload: &mut Cursor<Bytes>) -> u32 {
    payload.get_u32::<BigEndian>()
}


fn decode_longlong(payload: &mut Cursor<Bytes>) -> u64 {
    payload.get_u64::<BigEndian>()
}


fn decode_short_str(payload: &mut Cursor<Bytes>) -> String {
    let len = payload.get_u8();
    decode_string_with_length(payload, len as usize)
}


fn decode_long_str(payload: &mut Cursor<Bytes>) -> String {
    let len = payload.get_u32::<BigEndian>();
    decode_string_with_length(payload, len as usize)
}


fn decode_field_table(mut cursor: &mut Cursor<Bytes>) -> HashMap<String, FieldArgument> {
    debug!("decode field table");

    let size = cursor.get_u32::<BigEndian>() as u64;

    let mut table = HashMap::new();
    let start_pos = cursor.position();

    while (cursor.position() - start_pos) < size {
        let item_name = decode_short_str(cursor);
        let item_value = decode_field_item_value(cursor);
        table.insert(item_name, item_value);
    }

    table
}


/*
fn decode_field_array(cursor: &mut Cursor<Bytes>) -> Result<FieldArray> {
    let size = cursor.get_u32::<BigEndian>() as u64;

    let mut items = Vec::new();
    let start_pos = cursor.position();

    while (cursor.position() - start_pos) < size {
        let item = decode_field_item_value(cursor);
        items.push(item);
    }
    Ok(FieldArray(items))
}
*/


fn decode_field_item_value(cursor: &mut Cursor<Bytes>) -> FieldArgument {
    match cursor.get_u8() {
        0x74 => FieldArgument::Boolean(cursor.get_u8() == 0x01),
        0x62 => FieldArgument::SignedOctet(cursor.get_i8()),
        0x42 => FieldArgument::UnsignedOctet(cursor.get_u8()),
        0x55 => FieldArgument::SignedShort(cursor.get_i16::<BigEndian>()),
        0x75 => FieldArgument::UnsignedShort(cursor.get_u16::<BigEndian>()),
        0x49 => FieldArgument::SignedLong(cursor.get_i32::<BigEndian>()),
        0x69 => FieldArgument::UnsignedLong(cursor.get_u32::<BigEndian>()),
        0x4C => FieldArgument::SignedLongLong(cursor.get_i64::<BigEndian>()),
        0x6C => FieldArgument::UnsignedLongLong(cursor.get_u64::<BigEndian>()),
        0x66 => FieldArgument::Float(cursor.get_f32::<BigEndian>()),
        0x63 => FieldArgument::Double(cursor.get_f64::<BigEndian>()),
        0x44 => FieldArgument::Decimal(cursor.get_i64::<BigEndian>()),
        0x73 => {
            let len = cursor.get_u8();
            FieldArgument::ShortString(decode_string_with_length(cursor, len as usize))
        }
        0x53 => {
            let len = cursor.get_u32::<BigEndian>();
            FieldArgument::LongString(decode_string_with_length(cursor, len as usize))
        }
        0x54 => FieldArgument::Timestamp(cursor.get_u64::<BigEndian>()),
        0x46 => FieldArgument::NestedTable(decode_field_table(cursor)),
        0x56 => FieldArgument::Void,
        0x78 => panic!(), // I don't know how should I treat it
        b => unreachable!("Unexpected byte {} at decode_field_item_value", b),
    }
}


fn decode_string_with_length(cursor: &mut Cursor<Bytes>, length: usize) -> String {
    let string = ::std::str::from_utf8(&cursor.bytes()[0..length])
        .expect(
            format!("Non UTF8 bytes {:?}", &cursor.bytes()[0..length]).as_str(),
        )
        .into();
    let pos = cursor.position();
    cursor.set_position(pos + length as u64);
    debug!("decode string : {}", string);
    string
}
// }}}
