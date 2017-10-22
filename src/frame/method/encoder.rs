use bytes::{BufMut, BigEndian};

use std::collections::HashMap;

use super::{MethodPayload, ConnectionClass, ChannelClass, ExchangeClass, QueueClass, BasicClass,
            TxClass};
use args::*;


pub fn encode_payload(payload: MethodPayload) -> Vec<u8> {
    use self::MethodPayload::*;
    match payload {
        Connection(class) => encode_connection_class(class),
        Channel(class) => encode_channel_class(class),
        Exchange(class) => encode_exchange_class(class),
        Queue(class) => encode_queue_class(class),
        Basic(class) => encode_basic_class(class),
        Tx(class) => encode_tx_class(class),
    }
}


struct InnerEncoder {
    buf: Vec<u8>,
}


// Encode Connection Class {{{
fn encode_connection_class(class: ConnectionClass) -> Vec<u8> {
    const CLASS_ID: u16 = 10;
    use self::ConnectionClass::*;
    match class {
        StartOk(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 11)
                .encode_field_table(m.client_properties)
                .encode_short_str(m.mechanism)
                .encode_long_str(m.response)
                .encode_short_str(m.locale)
                .vec()
        }
        SecureOk(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 21)
                .encode_long_str(m.response)
                .vec()
        }
        TuneOk(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 31)
                .encode_short(m.channel_max)
                .encode_long(m.frame_max)
                .encode_short(m.heartbeat)
                .vec()
        }
        Open(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 40)
                .encode_short_str(m.virtual_host)
                .encode_short_str(m.reserved1)
                .encode_bit_1(m.reserved2)
                .vec()
        }
        Close(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 50)
                .encode_short(m.reply_code)
                .encode_short_str(m.reply_text)
                .encode_short(m.class_id)
                .encode_short(m.method_id)
                .vec()
        }
        CloseOk => InnerEncoder::class_and_method_id(CLASS_ID, 51).vec(),
        _ => unreachable!("Others methods are never be sent by client"),
    }
}
// }}}


// Encode Channel Class  {{{
fn encode_channel_class(class: ChannelClass) -> Vec<u8> {
    const CLASS_ID: u16 = 20;
    use self::ChannelClass::*;
    match class {
        Open(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 10)
                .encode_short_str(m.reserved1)
                .vec()
        }
        Flow(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 20)
                .encode_bit_1(m.active)
                .vec()
        }
        FlowOk(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 21)
                .encode_bit_1(m.active)
                .vec()
        }
        Close(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 40)
                .encode_short(m.reply_code)
                .encode_short_str(m.reply_text)
                .encode_short(m.class_id)
                .encode_short(m.method_id)
                .vec()
        }
        CloseOk => InnerEncoder::class_and_method_id(CLASS_ID, 41).vec(),
        _ => unreachable!("Others methods are never be sent by client"),
    }
}
// }}}


// Encode Exchange Class {{{
fn encode_exchange_class(class: ExchangeClass) -> Vec<u8> {
    const CLASS_ID: u16 = 40;
    use self::ExchangeClass::*;
    match class {
        Declare(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 10)
                .encode_short(m.reserved1)
                .encode_short_str(m.exchange)
                .encode_short_str(m.typ)
                .encode_bit_5(m.passive, m.durable, m.auto_delete, m.internal, m.no_wait)
                .encode_field_table(m.arguments)
                .vec()
        }
        Delete(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 20)
                .encode_short(m.reserved1)
                .encode_short_str(m.exchange)
                .encode_bit_2(m.if_unused, m.no_wait)
                .vec()
        }
        Bind(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 30)
                .encode_short(m.reserved1)
                .encode_short_str(m.destination)
                .encode_short_str(m.source)
                .encode_short_str(m.routing_key)
                .encode_bit_1(m.no_wait)
                .encode_field_table(m.arguments)
                .vec()
        }
        Unbind(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 40)
                .encode_short(m.reserved1)
                .encode_short_str(m.destination)
                .encode_short_str(m.source)
                .encode_short_str(m.routing_key)
                .encode_bit_1(m.no_wait)
                .encode_field_table(m.arguments)
                .vec()
        }
        _ => unreachable!("Others methods are never be sent by client"),
    }
}
// }}}


// Encode Queue Class {{{
fn encode_queue_class(class: QueueClass) -> Vec<u8> {
    const CLASS_ID: u16 = 50;
    use self::QueueClass::*;
    match class {
        Declare(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 10)
                .encode_short(m.reserved1)
                .encode_short_str(m.queue)
                .encode_bit_5(m.passive, m.durable, m.exclusive, m.auto_delete, m.no_wait)
                .encode_field_table(m.arguments)
                .vec()
        }
        Bind(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 20)
                .encode_short(m.reserved1)
                .encode_short_str(m.queue)
                .encode_short_str(m.exchange)
                .encode_short_str(m.routing_key)
                .encode_bit_1(m.no_wait)
                .encode_field_table(m.arguments)
                .vec()
        }
        Unbind(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 50)
                .encode_short(m.reserved1)
                .encode_short_str(m.queue)
                .encode_short_str(m.exchange)
                .encode_short_str(m.routing_key)
                .encode_field_table(m.arguments)
                .vec()
        }
        Purge(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 30)
                .encode_short(m.reserved1)
                .encode_short_str(m.queue)
                .encode_bit_1(m.no_wait)
                .vec()
        }
        Delete(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 40)
                .encode_short(m.reserved1)
                .encode_short_str(m.queue)
                .encode_bit_3(m.if_unused, m.if_empty, m.no_wait)
                .vec()
        }
        _ => unreachable!("Others methods are never be sent by client"),
    }
}
// }}}


// Encode Basic Class {{{
fn encode_basic_class(class: BasicClass) -> Vec<u8> {
    const CLASS_ID: u16 = 60;
    use self::BasicClass::*;
    match class {
        Qos(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 10)
                .encode_long(m.prefetch_size)
                .encode_short(m.prefetch_count)
                .encode_bit_1(m.global)
                .vec()
        }
        Consume(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 20)
                .encode_short(m.reserved1)
                .encode_short_str(m.queue)
                .encode_short_str(m.consumer_tag)
                .encode_bit_4(m.no_local, m.no_ack, m.exclusive, m.no_wait)
                .encode_field_table(m.arguments)
                .vec()
        }
        Cancel(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 30)
                .encode_short_str(m.consumer_tag)
                .encode_bit_1(m.no_wait)
                .vec()
        }
        Publish(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 40)
                .encode_short(m.reserved1)
                .encode_short_str(m.exchange)
                .encode_short_str(m.routing_key)
                .encode_bit_2(m.mandatory, m.immediate)
                .vec()
        }
        Get(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 70)
                .encode_short(m.reserved1)
                .encode_short_str(m.queue)
                .encode_bit_1(m.no_ack)
                .vec()
        }
        Ack(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 80)
                .encode_longlong(m.delivery_tag)
                .encode_bit_1(m.multiple)
                .vec()
        }
        Reject(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 90)
                .encode_longlong(m.delivery_tag)
                .encode_bit_1(m.requeue)
                .vec()
        }
        Nack(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 120)
                .encode_longlong(m.delivery_tag)
                .encode_bit_1(m.multiple)
                .vec()
        }
        RecoverAsync(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 100)
                .encode_bit_1(m.requeue)
                .vec()
        }
        Recover(m) => {
            InnerEncoder::class_and_method_id(CLASS_ID, 110)
                .encode_bit_1(m.requeue)
                .vec()
        }
        _ => unreachable!("Others methods are never be sent by client"),
    }
}
// }}}


// Encode Tx Class {{{
fn encode_tx_class(class: TxClass) -> Vec<u8> {
    const CLASS_ID: u16 = 90;
    use self::TxClass::*;
    match class {
        Select => InnerEncoder::class_and_method_id(CLASS_ID, 10).vec(),
        Commit => InnerEncoder::class_and_method_id(CLASS_ID, 20).vec(),
        Rollback => InnerEncoder::class_and_method_id(CLASS_ID, 30).vec(),
        _ => unreachable!("Others methods are never be sent by client"),
    }
}
// }}}



// impl InnerEncoder {{{
impl InnerEncoder {
    fn class_and_method_id(class_id: u16, method_id: u16) -> InnerEncoder {
        const INITIAL_CAPACITY: usize = 8;

        let mut buf = Vec::with_capacity(INITIAL_CAPACITY);
        buf.put_u16::<BigEndian>(class_id);
        buf.put_u16::<BigEndian>(method_id);

        InnerEncoder { buf: buf }
    }

    // For now, any encoded method does not have octet field
    #[allow(dead_code)]
    fn encode_octet(mut self, octet: u8) -> InnerEncoder {
        self.buf.put_u8(octet);
        self
    }

    fn encode_short(mut self, short: u16) -> InnerEncoder {
        self.buf.put_u16::<BigEndian>(short);
        self
    }

    fn encode_long(mut self, long: u32) -> InnerEncoder {
        self.buf.put_u32::<BigEndian>(long);
        self
    }

    fn encode_longlong(mut self, longlong: u64) -> InnerEncoder {
        self.buf.put_u64::<BigEndian>(longlong);
        self
    }

    fn encode_bit_1(self, bit: bool) -> InnerEncoder {
        self.encode_bit_5(bit, false, false, false, false)
    }

    fn encode_bit_2(self, bit1: bool, bit2: bool) -> InnerEncoder {
        self.encode_bit_5(bit1, bit2, false, false, false)
    }

    fn encode_bit_3(self, bit1: bool, bit2: bool, bit3: bool) -> InnerEncoder {
        self.encode_bit_5(bit1, bit2, bit3, false, false)
    }

    fn encode_bit_4(self, bit1: bool, bit2: bool, bit3: bool, bit4: bool) -> InnerEncoder {
        self.encode_bit_5(bit1, bit2, bit3, bit4, false)
    }

    fn encode_bit_5(
        mut self,
        bit1: bool,
        bit2: bool,
        bit3: bool,
        bit4: bool,
        bit5: bool,
    ) -> InnerEncoder {
        let byte = (bit1 as u8 * 0b_0000_0001) + (bit2 as u8 * 0b_0000_0010) +
            (bit3 as u8 * 0b_0000_0100) + (bit4 as u8 * 0b_0000_1000) +
            (bit5 as u8 * 0b_0001_0000);
        self.buf.put_u8(byte);
        self
    }

    fn encode_short_str(mut self, string: String) -> InnerEncoder {
        self.buf.put_u8(string.len() as u8);
        self.buf.put(string.as_bytes());
        self
    }

    fn encode_long_str(mut self, string: String) -> InnerEncoder {
        self.buf.put_u32::<BigEndian>(string.len() as u32);
        self.buf.put(string.as_bytes());
        self
    }

    fn encode_field_table(mut self, table: HashMap<String, FieldArgument>) -> InnerEncoder {
        encode_field_table_0(&table, &mut self.buf);
        self
    }

    fn vec(self) -> Vec<u8> {
        self.buf
    }
}
// }}}


// Encode field-table and field-array {{{
fn encode_field_table_0(table: &HashMap<String, FieldArgument>, dst: &mut Vec<u8>) {
    let mut bytes = {
        let mut buf = Vec::new();
        for (item_name, item_value) in table.iter() {
            buf.put_u8(item_name.len() as u8);
            buf.put(item_name);
            encode_field_item(item_value, &mut buf);
        }
        buf
    };

    dst.put_u32::<BigEndian>(bytes.len() as u32);
    dst.append(&mut bytes);
}


/*
fn encode_field_array(array: &FieldArray, dst: &mut Vec<u8>) {
    let mut bytes = {
        let mut buf = Vec::new();
        for item in array.iter() {
            encode_field_item(item, &mut buf);
        }
        buf
    };

    dst.put_u32::<BigEndian>(bytes.len() as u32);
    dst.append(&mut bytes);
}
*/


fn encode_field_item(item: &FieldArgument, dst: &mut Vec<u8>) {
    match item {
        &FieldArgument::Boolean(b) => {
            dst.put_u8(b't');
            dst.put_u8(b as u8);
        }
        &FieldArgument::SignedOctet(byte) => {
            dst.put_u8(b'b');
            dst.put_i8(byte);
        }
        &FieldArgument::UnsignedOctet(byte) => {
            dst.put_u8(b'B');
            dst.put_u8(byte);
        }
        &FieldArgument::SignedShort(short) => {
            dst.put_u8(b'U');
            dst.put_i16::<BigEndian>(short);
        }
        &FieldArgument::UnsignedShort(short) => {
            dst.put_u8(b'u');
            dst.put_u16::<BigEndian>(short);
        }
        &FieldArgument::SignedLong(long) => {
            dst.put_u8(b'I');
            dst.put_i32::<BigEndian>(long);
        }
        &FieldArgument::UnsignedLong(long) => {
            dst.put_u8(b'i');
            dst.put_u32::<BigEndian>(long);
        }
        &FieldArgument::SignedLongLong(longlong) => {
            dst.put_u8(b'L');
            dst.put_i64::<BigEndian>(longlong);
        }
        &FieldArgument::UnsignedLongLong(longlong) => {
            dst.put_u8(b'l');
            dst.put_u64::<BigEndian>(longlong);
        }
        &FieldArgument::Float(float) => {
            dst.put_u8(b'f');
            dst.put_f32::<BigEndian>(float);
        }
        &FieldArgument::Double(double) => {
            dst.put_u8(b'd');
            dst.put_f64::<BigEndian>(double);
        }
        &FieldArgument::Decimal(decimal) => {
            dst.put_u8(b'D');
            dst.put_i64::<BigEndian>(decimal);
        }
        &FieldArgument::ShortString(ref s) => {
            dst.put_u8(b's');
            dst.put_u8(s.len() as u8);
            dst.put(s.as_bytes());
        }
        &FieldArgument::LongString(ref s) => {
            dst.put_u8(b'S');
            dst.put_u32::<BigEndian>(s.len() as u32);
            dst.put(s.as_bytes());
        }
        &FieldArgument::Timestamp(ts) => {
            dst.put_u8(b'T');
            dst.put_u64::<BigEndian>(ts);
        }
        &FieldArgument::NestedTable(ref table) => {
            dst.put_u8(b'F');
            encode_field_table_0(table, dst);
        }
        &FieldArgument::Void => {
            dst.put_u8(b'V');
        }
        &FieldArgument::ByteArray(ref _array) => {
            dst.put_u8(b'x');
            panic!("Fail to parse ByteArray") // I don't know how should I treat it
        }
    }
}
// }}}
