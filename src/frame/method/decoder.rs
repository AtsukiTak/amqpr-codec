use bytes::{BytesMut, BigEndian, Buf};

use std::io::Cursor;
use std::collections::HashMap;

use frame::method::{MethodPayload, ConnectionClass, ChannelClass, ExchangeClass, QueueClass,
                    BasicClass, TxClass};
use args::*;

/// # NOTICE
/// This method does not check payload length.
pub fn decode_payload(bytes: &mut BytesMut) -> MethodPayload {
    let mut cursor = Cursor::new(bytes.split_off(4).freeze());

    let class_id = cursor.get_u16::<BigEndian>();
    debug!("class_id is {}", class_id);

    let method_id = cursor.get_u16::<BigEndian>();
    debug!("method_id is {}", method_id);

    drop(cursor);

    match class_id {
        10 => MethodPayload::Connection(decode_connection_class(method_id, bytes)),
        20 => MethodPayload::Channel(decode_channel_class(method_id, bytes)),
        40 => MethodPayload::Exchange(decode_exchange_class(method_id, bytes)),
        50 => MethodPayload::Queue(decode_queue_class(method_id, bytes)),
        60 => MethodPayload::Basic(decode_basic_class(method_id, bytes)),
        90 => MethodPayload::Tx(decode_tx_class(method_id, bytes)),
        c => unreachable!("Unexpected class id {}", c),
    }
}


// Decode Connection Class {{{
fn decode_connection_class(method_id: u16, bytes: &mut BytesMut) -> ConnectionClass {
    use frame::method::connection::*;
    use self::ConnectionClass::*;
    match method_id {
        10 => Start(StartMethod {
            version_major: decode_octet(bytes),
            version_minor: decode_octet(bytes),
            server_properties: decode_field_table(bytes),
            mechanisms: decode_long_str(bytes),
            locales: decode_long_str(bytes),
        }),
        20 => Secure(SecureMethod { challenge: decode_long_str(bytes) }),
        30 => Tune(TuneMethod {
            channel_max: decode_short(bytes),
            frame_max: decode_long(bytes),
            heartbeat: decode_short(bytes),
        }),
        41 => OpenOk(OpenOkMethod { reserved1: decode_short_str(bytes) }),
        50 => Close(CloseMethod {
            reply_code: decode_short(bytes),
            reply_text: decode_short_str(bytes),
            class_id: decode_short(bytes),
            method_id: decode_short(bytes),
        }),
        51 => CloseOk,
        60 => Blocked(BlockedMethod { reason: decode_short_str(bytes) }),
        61 => Unblocked,
        m => unreachable!("Unexpected method id {} in Connection class", m),
    }
}
// }}}


// Decode Channel Class {{{
fn decode_channel_class(method_id: u16, bytes: &mut BytesMut) -> ChannelClass {
    use frame::method::channel::*;
    use self::ChannelClass::*;
    match method_id {
        11 => OpenOk(OpenOkMethod { reserved1: decode_long_str(bytes) }),
        20 => Flow(FlowMethod { active: decode_bool_1(bytes) }),
        21 => FlowOk(FlowOkMethod { active: decode_bool_1(bytes) }),
        40 => Close(CloseMethod {
            reply_code: decode_short(bytes),
            reply_text: decode_short_str(bytes),
            class_id: decode_short(bytes),
            method_id: decode_short(bytes),
        }),
        41 => CloseOk,
        m => unreachable!("Unexpected method id {} in Channel class", m),
    }
}
// }}}


// Decode Exchange Class {{{
fn decode_exchange_class(method_id: u16, _payload: &mut BytesMut) -> ExchangeClass {
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
fn decode_queue_class(method_id: u16, bytes: &mut BytesMut) -> QueueClass {
    use frame::method::queue::*;
    use self::QueueClass::*;
    match method_id {
        11 => DeclareOk(DeclareOkMethod {
            queue: decode_short_str(bytes),
            message_count: decode_long(bytes),
            consumer_count: decode_long(bytes),
        }),
        21 => BindOk,
        31 => PurgeOk(PurgeOkMethod { message_count: decode_long(bytes) }),
        41 => DeleteOk(DeleteOkMethod { message_count: decode_long(bytes) }),
        51 => UnbindOk,
        m => unreachable!("Unexpected method id {} in Channel class", m),
    }
}
// }}}


// Decode Basic Class {{{
fn decode_basic_class(method_id: u16, bytes: &mut BytesMut) -> BasicClass {
    use frame::method::basic::*;
    use self::BasicClass::*;
    match method_id {
        11 => QosOk,
        21 => ConsumeOk(ConsumeOkMethod { consumer_tag: decode_short_str(bytes) }),
        31 => CancelOk(CancelOkMethod { consumer_tag: decode_short_str(bytes) }),
        50 => Return(ReturnMethod {
            reply_code: decode_short(bytes),
            reply_text: decode_short_str(bytes),
            exchange: decode_short_str(bytes),
            routing_key: decode_short_str(bytes),
        }),
        60 => Deliver(DeliverMethod {
            consumer_tag: decode_short_str(bytes),
            delivery_tag: decode_longlong(bytes),
            redeliverd: decode_bool_1(bytes),
            exchange: decode_short_str(bytes),
            routing_key: decode_short_str(bytes),
        }),
        71 => GetOk(GetOkMethod {
            delivery_tag: decode_longlong(bytes),
            redeliverd: decode_bool_1(bytes),
            exchange: decode_short_str(bytes),
            routing_key: decode_short_str(bytes),
            message_count: decode_long(bytes),
        }),
        72 => GetEmpty(GetEmptyMethod { reserved1: decode_short_str(bytes) }),
        80 => Ack(AckMethod {
            delivery_tag: decode_longlong(bytes),
            multiple: decode_bool_1(bytes),
        }),

        // rabbitmq-specific extension
        120 => Nack(NackMethod {
            delivery_tag: decode_longlong(bytes),
            multiple: decode_bool_1(bytes),
        }),

        m => unreachable!("Unexpected method id {} in Channel class", m),
    }
}
// }}}


// Decode Tx Class {{{
fn decode_tx_class(method_id: u16, _bytes: &mut BytesMut) -> TxClass {
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
fn decode_bool_1(bytes: &mut BytesMut) -> bool {
    bytes.split_to(1)[0] & 0b_1000_0000 == 0b_1000_0000
}


fn decode_octet(bytes: &mut BytesMut) -> u8 {
    bytes.split_to(1)[0]
}


fn decode_short(bytes: &mut BytesMut) -> u16 {
    Cursor::new(bytes.split_to(2)).get_u16::<BigEndian>()
}


fn decode_long(bytes: &mut BytesMut) -> u32 {
    Cursor::new(bytes.split_to(4)).get_u32::<BigEndian>()
}


fn decode_longlong(bytes: &mut BytesMut) -> u64 {
    Cursor::new(bytes.split_to(8)).get_u64::<BigEndian>()
}


fn decode_short_str(bytes: &mut BytesMut) -> AmqpString {
    let len = bytes.split_to(1)[0];
    AmqpString(bytes.split_to(len as usize).freeze())
}


fn decode_long_str(bytes: &mut BytesMut) -> AmqpString {
    let len = Cursor::new(bytes.split_to(4)).get_u32::<BigEndian>();
    AmqpString(bytes.split_to(len as usize).freeze())
}


pub(crate) fn decode_field_table(bytes: &mut BytesMut) -> HashMap<AmqpString, FieldArgument> {
    debug!("decode field table");

    let size = Cursor::new(bytes.split_to(4)).get_u32::<BigEndian>() as u64;

    let mut bytes = bytes.split_to(size as usize);

    let mut table = HashMap::new();

    while bytes.len() > 0 {
        let item_name = decode_short_str(&mut bytes);
        let item_value = decode_field_item_value(&mut bytes);
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


fn decode_field_item_value(bytes: &mut BytesMut) -> FieldArgument {
    let flag = bytes.split_to(1)[0];
    match flag {
        0x74 => FieldArgument::Boolean(bytes.split_to(1)[0] == 0x01),
        0x62 => FieldArgument::SignedOctet(Cursor::new(bytes.split_to(1)).get_i8()),
        0x42 => FieldArgument::UnsignedOctet(bytes.split_to(1)[0]),
        0x55 => FieldArgument::SignedShort(Cursor::new(bytes.split_to(2)).get_i16::<BigEndian>()),
        0x75 => FieldArgument::UnsignedShort(Cursor::new(bytes.split_to(2)).get_u16::<BigEndian>()),
        0x49 => FieldArgument::SignedLong(Cursor::new(bytes.split_to(4)).get_i32::<BigEndian>()),
        0x69 => FieldArgument::UnsignedLong(Cursor::new(bytes.split_to(4)).get_u32::<BigEndian>()),
        0x4C => FieldArgument::SignedLongLong(
            Cursor::new(bytes.split_to(8)).get_i64::<BigEndian>(),
        ),
        0x6C => FieldArgument::UnsignedLongLong(
            Cursor::new(bytes.split_to(8)).get_u64::<BigEndian>(),
        ),
        0x66 => FieldArgument::Float(Cursor::new(bytes.split_to(4)).get_f32::<BigEndian>()),
        0x63 => FieldArgument::Double(Cursor::new(bytes.split_to(8)).get_f64::<BigEndian>()),
        0x44 => FieldArgument::Decimal(Cursor::new(bytes.split_to(8)).get_i64::<BigEndian>()),
        0x73 => {
            let len = bytes.split_to(1)[0];
            FieldArgument::ShortString(AmqpString(bytes.split_to(len as usize).freeze()))
        }
        0x53 => {
            let len = Cursor::new(bytes.split_to(4)).get_u32::<BigEndian>();
            FieldArgument::LongString(AmqpString(bytes.split_to(len as usize).freeze()))
        }
        0x54 => FieldArgument::Timestamp(Cursor::new(bytes.split_to(8)).get_u64::<BigEndian>()),
        0x46 => FieldArgument::NestedTable(decode_field_table(bytes)),
        0x56 => FieldArgument::Void,
        0x78 => panic!(), // I don't know how should I treat it
        b => unreachable!("Unexpected byte {} at decode_field_item_value", b),
    }
}
// }}}
