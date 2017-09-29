use bytes::{Bytes, BigEndian, Buf};

use std::io::Cursor;
use std::iter::IntoIterator;

use frame::method::MethodPayload;
use methods::args::*;
use errors::*;

/// # NOTICE
/// This method does not check payload length.
pub fn decode_payload(cursor: &mut Cursor<Bytes>) -> Result<MethodPayload> {
    let class_id = cursor.get_u16::<BigEndian>();
    debug!("class_id is {}", class_id);

    let method_id = cursor.get_u16::<BigEndian>();
    debug!("method_id is {}", method_id);

    let arguments = decode_arguments(cursor, class_id, method_id)?;

    Ok(MethodPayload {
        class_id: class_id,
        method_id: method_id,
        arguments: arguments,
    })
}


pub fn decode_arguments(payload: &mut Cursor<Bytes>,
                        class_id: u16,
                        method_id: u16)
                        -> Result<Arguments> {

    fn inner<'a, I>(payload: &mut Cursor<Bytes>, infos: I) -> Result<Arguments>
        where I: IntoIterator<Item = &'a ArgumentDecodingInfo>
    {
        let mut arguments = Vec::new();
        for info in infos {
            arguments.push(decode_argument_with_info(payload, info)?);
        }
        Ok(Arguments(arguments))
    }

    use self::ArgumentDecodingInfo::*;
    match (class_id, method_id) {
        (10, 10) => inner(payload, &[Octet, Octet, FieldTable, LongString, LongString]),
        (10, 30) => inner(payload, &[Short, Long, Short]),
        (10, 41) => inner(payload, &[ShortString]),
        (10, 50) => inner(payload, &[Short, ShortString, Short, Short]),
        (20, 11) => inner(payload, &[LongString]),
        (20, 40) => inner(payload, &[Short, ShortString, Short, Short]),
        (40, 11) => inner(payload, &[]),
        (50, 11) => inner(payload, &[ShortString, Long, Long]),
        (50, 21) => inner(payload, &[]),
        (60, 21) => inner(payload, &[ShortString]),
        (60, 60) => {
            inner(payload,
                  &[ShortString, LongLong, Bits, ShortString, ShortString])
        }
        _ => Err(ErrorKind::UnknownClassIdOrMethodId(class_id, method_id).into()),
    }


}


pub fn decode_argument_with_info(payload: &mut Cursor<Bytes>,
                                 symbol: &ArgumentDecodingInfo)
                                 -> Result<Argument> {

    let arg = match symbol {
        &ArgumentDecodingInfo::Octet => Argument::Octet(payload.get_u8()),
        &ArgumentDecodingInfo::Short => Argument::Short(payload.get_u16::<BigEndian>()),
        &ArgumentDecodingInfo::Long => Argument::Long(payload.get_u32::<BigEndian>()),
        &ArgumentDecodingInfo::LongLong => Argument::LongLong(payload.get_u64::<BigEndian>()),
        &ArgumentDecodingInfo::Bits => Argument::Bits(payload.get_u8()),
        &ArgumentDecodingInfo::ShortString => {
            let len = payload.get_u8() as usize;
            Argument::ShortString(ShortString(decode_string_with_length(payload, len)?))
        }
        &ArgumentDecodingInfo::LongString => {
            let len = payload.get_u32::<BigEndian>() as usize;
            Argument::LongString(LongString(decode_string_with_length(payload, len)?))
        }
        &ArgumentDecodingInfo::Timestamp => {
            Argument::Timestamp(Timestamp(payload.get_u64::<BigEndian>()))
        }
        &ArgumentDecodingInfo::FieldTable => Argument::FieldTable(decode_field_table(payload)?),
        &ArgumentDecodingInfo::FieldArray => Argument::FieldArray(decode_field_array(payload)?),
    };
    Ok(arg)
}



fn decode_field_table(mut cursor: &mut Cursor<Bytes>) -> Result<FieldTable> {
    debug!("decode field table");

    let size = cursor.get_u32::<BigEndian>() as u64;

    let mut items = Vec::new();
    let start_pos = cursor.position();

    while (cursor.position() - start_pos) < size {
        let item_name = decode_field_item_name(cursor)?;
        let item_value = decode_field_item_value(cursor)?;
        items.push((item_name, item_value));
    }

    Ok(FieldTable(items))
}


fn decode_field_array(cursor: &mut Cursor<Bytes>) -> Result<FieldArray> {
    let size = cursor.get_u32::<BigEndian>() as u64;

    let mut items = Vec::new();
    let start_pos = cursor.position();

    while (cursor.position() - start_pos) < size {
        let item = decode_field_item_value(cursor)?;
        items.push(item);
    }
    Ok(FieldArray(items))
}


fn decode_field_item_name(cursor: &mut Cursor<Bytes>) -> Result<String> {
    let item_name_size = cursor.get_u8();
    decode_string_with_length(cursor, item_name_size as usize)
}


fn decode_field_item_value(cursor: &mut Cursor<Bytes>) -> Result<FieldArgument> {
    let arg = match cursor.get_u8() {
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
            FieldArgument::ShortString(ShortString(decode_string_with_length(cursor,
                                                                             len as usize)?))
        }
        0x53 => {
            let len = cursor.get_u32::<BigEndian>();
            FieldArgument::LongString(LongString(decode_string_with_length(cursor, len as usize)?))
        }
        0x54 => FieldArgument::Timestamp(cursor.get_u64::<BigEndian>()),
        0x46 => FieldArgument::NestedTable(decode_field_table(cursor)?),
        0x56 => FieldArgument::Void,
        0x78 => panic!(), // I don't know how should I treat it
        b => return Err(ErrorKind::InvalidFieldArgumentTypeByte(b).into()),
    };
    Ok(arg)
}


fn decode_string_with_length(cursor: &mut Cursor<Bytes>, length: usize) -> Result<String> {
    let string = ::std::str::from_utf8(&cursor.bytes()[0..length])?.into();
    let pos = cursor.position();
    cursor.set_position(pos + length as u64);
    debug!("decode string : {}", string);
    Ok(string)
}
