use bytes::{BufMut, BigEndian};

use super::MethodPayload;
use methods::args::*;
use errors::*;

pub fn encode_payload(payload: MethodPayload, dst: &mut Vec<u8>) -> Result<()> {
    // reserve for class_id and method_id
    dst.reserve(4);

    // put class_id and method_id
    dst.put_u16::<BigEndian>(payload.class_id);
    dst.put_u16::<BigEndian>(payload.method_id);

    // put arguments
    for arg in payload.arguments.iter() {
        encode_argument(arg, dst)?;
    }
    Ok(())
}


pub fn encode_argument(arg: &Argument, dst: &mut Vec<u8>) -> Result<()> {
    match arg {
        &Argument::Octet(byte) => {
            dst.reserve(1);
            dst.put_u8(byte);
        }
        &Argument::Short(short) => {
            dst.reserve(2);
            dst.put_u16::<BigEndian>(short);
        }
        &Argument::Long(long) => {
            dst.reserve(4);
            dst.put_u32::<BigEndian>(long);
        }
        &Argument::LongLong(longlong) => {
            dst.reserve(8);
            dst.put_u64::<BigEndian>(longlong);
        }
        &Argument::Bits(ref byte) => {
            dst.reserve(1);
            dst.put_u8(byte.clone());
        }
        &Argument::ShortString(ref string) => {
            // Reserve length byte and string bytes.
            dst.reserve(byte_size_of_short_string(string) as usize);
            dst.put_u8(string.len() as u8);
            dst.put(&string.0);
        }
        &Argument::LongString(ref string) => {
            // Reserve length byte and string bytes.
            dst.reserve(byte_size_of_long_string(string) as usize);
            dst.put_u32::<BigEndian>(string.len() as u32);
            dst.put(&string.0);
        }
        &Argument::Timestamp(Timestamp(t)) => {
            dst.reserve(8);
            dst.put_u64::<BigEndian>(t);
        }
        &Argument::FieldTable(ref t) => {
            encode_field_table(t, dst)?;
        }
        &Argument::FieldArray(ref a) => {
            encode_field_array(a, dst)?;
        }
    }
    Ok(())
}


fn encode_field_table(table: &FieldTable, dst: &mut Vec<u8>) -> Result<()> {
    let reserve_size = byte_size_of_field_table(table);
    dst.reserve(reserve_size as usize);

    let size = reserve_size - 4_u32; // Because `byte_size_of_field_table` contains a size of size.
    debug!("Field table byte size : {}", size);
    dst.put_u32::<BigEndian>(size);
    for &(ref item_name, ref item_value) in table.iter() {
        dst.put_u8(item_name.len() as u8);
        dst.put(item_name);
        encode_field_item(item_value, dst)?;
    }
    Ok(())
}


/// You should reserve capacity of dst before pass to here.
fn encode_field_array(array: &FieldArray, dst: &mut Vec<u8>) -> Result<()> {
    let reserve_size = byte_size_of_field_array(array); // Because `byte_size_of_field_table` contains a size of size.
    dst.reserve(reserve_size as usize);
    dst.put_u32::<BigEndian>(reserve_size - 4);
    for item in array.iter() {
        encode_field_item(item, dst)?;
    }
    Ok(())
}


/// You should reserve capacity of dst before pass to here.
fn encode_field_item(item: &FieldArgument, dst: &mut Vec<u8>) -> Result<()> {
    match item {
        &FieldArgument::Boolean(b) => {
            dst.put_u8(0x74); // 0x74 represents "t"
            dst.put_u8(b as u8);
        }
        &FieldArgument::SignedOctet(byte) => {
            dst.put_u8(0x62); // 0x62 represents "b"
            dst.put_i8(byte);
        }
        &FieldArgument::UnsignedOctet(byte) => {
            dst.put_u8(0x42); // 0x42 represents "B"
            dst.put_u8(byte);
        }
        &FieldArgument::SignedShort(short) => {
            dst.put_u8(0x55); // 0x55 represents "U"
            dst.put_i16::<BigEndian>(short);
        }
        &FieldArgument::UnsignedShort(short) => {
            dst.put_u8(0x75); // 0x75 represents "u"
            dst.put_u16::<BigEndian>(short);
        }
        &FieldArgument::SignedLong(long) => {
            dst.put_u8(0x49); // 0x49 represents "I"
            dst.put_i32::<BigEndian>(long);
        }
        &FieldArgument::UnsignedLong(long) => {
            dst.put_u8(0x69); // 0x69 represents "i"
            dst.put_u32::<BigEndian>(long);
        }
        &FieldArgument::SignedLongLong(longlong) => {
            dst.put_u8(0x4C); // 0x4C represents "L"
            dst.put_i64::<BigEndian>(longlong);
        }
        &FieldArgument::UnsignedLongLong(longlong) => {
            dst.put_u8(0x6C); // 0x6C represents "l"
            dst.put_u64::<BigEndian>(longlong);
        }
        &FieldArgument::Float(float) => {
            dst.put_u8(0x66); // 0x66 represents "f"
            dst.put_f32::<BigEndian>(float);
        }
        &FieldArgument::Double(double) => {
            dst.put_u8(0x63); // 0x63 represents "d"
            dst.put_f64::<BigEndian>(double);
        }
        &FieldArgument::Decimal(decimal) => {
            dst.put_u8(0x44); // 0x44 represents "D"
            dst.put_i64::<BigEndian>(decimal);
        }
        &FieldArgument::ShortString(ref sstr) => {
            dst.put_u8(0x73); // 0x73 represents "s"
            dst.put_u8(sstr.len() as u8);
            dst.put(&sstr.0);
        }
        &FieldArgument::LongString(ref lstr) => {
            dst.put_u8(0x53); // 0x53 represents "S"
            dst.put_u32::<BigEndian>(lstr.len() as u32);
            dst.put(&lstr.0);
        }
        &FieldArgument::Timestamp(ts) => {
            dst.put_u8(0x54); // 0x54 represents "T"
            dst.put_u64::<BigEndian>(ts);
        }
        &FieldArgument::NestedTable(ref table) => {
            dst.put_u8(0x46); // 0x46 represents "F"
            encode_field_table(table, dst)?;
        }
        &FieldArgument::Void => {
            dst.put_u8(0x56); // 0x56 represents "V"
        }
        &FieldArgument::ByteArray(ref _array) => {
            dst.put_u8(0x78); // 0x78 represents "x"
            panic!("Fail to parse ByteArray") // I don't know how should I treat it
        }
    }
    Ok(())
}



// ------- BYTE SIZE FUNCTIONS --------


pub fn byte_size_of_payload(payload: &MethodPayload) -> u32 {
    2_u32 // size of bytes representing class_id.
    + 2_u32 // size of bytes representing method_id.
    + payload.arguments.iter().fold(0_u32, |size, arg| size + byte_size_of_argument(arg))
}


fn byte_size_of_argument(arg: &Argument) -> u32 {
    match arg {
        &Argument::Octet(_) => 1_u32,
        &Argument::Short(_) => 2_u32,
        &Argument::Long(_) => 4_u32,
        &Argument::LongLong(_) => 8_u32,
        &Argument::Bits(_) => 1_u32,
        &Argument::ShortString(ref string) => byte_size_of_short_string(string),
        &Argument::LongString(ref string) => byte_size_of_long_string(string),
        &Argument::Timestamp(Timestamp(_)) => 8_u32,
        &Argument::FieldTable(ref t) => byte_size_of_field_table(t),
        &Argument::FieldArray(ref a) => byte_size_of_field_array(a),
    }
}




fn byte_size_of_short_string(s: &ShortString) -> u32 {
    s.len() as u32 + 1_u32
}


fn byte_size_of_long_string(s: &LongString) -> u32 {
    s.len() as u32 + 4_u32
}


fn byte_size_of_field_table(table: &FieldTable) -> u32 {
    // 4_u32 means size of length bytes
    table.iter().fold(4_u32, |size, &(ref string, ref item)| {
        size
            + 1_u32 // the size of byte representing string size.
            + string.len() as u32 // the size of string.
            + 1_u32 // the size of byte representing item type.
            + byte_size_of_field_item(item)
    })
}


fn byte_size_of_field_array(array: &FieldArray) -> u32 {
    // 4_u32 means size of length bytes
    array.iter().fold(4_u32, |size, item| size + byte_size_of_field_item(item))
}


fn byte_size_of_field_item(item: &FieldArgument) -> u32 {
    match item {
        &FieldArgument::Boolean(_) => panic!(), // TODO
        &FieldArgument::SignedOctet(_) => 1_u32,
        &FieldArgument::UnsignedOctet(_) => 1_u32,
        &FieldArgument::SignedShort(_) => 2_u32,
        &FieldArgument::UnsignedShort(_) => 2_u32,
        &FieldArgument::SignedLong(_) => 4_u32,
        &FieldArgument::UnsignedLong(_) => 4_u32,
        &FieldArgument::SignedLongLong(_) => 8_u32,
        &FieldArgument::UnsignedLongLong(_) => 8_u32,
        &FieldArgument::Float(_) => 4_u32,
        &FieldArgument::Double(_) => 8_u32,
        &FieldArgument::Decimal(_) => 8_u32,
        &FieldArgument::ShortString(ref s) => byte_size_of_short_string(s),
        &FieldArgument::LongString(ref s) => byte_size_of_long_string(s),
        &FieldArgument::Timestamp(_) => 8_u32,
        &FieldArgument::NestedTable(ref f) => byte_size_of_field_table(f),
        &FieldArgument::Void => 0_u32,
        &FieldArgument::ByteArray(ref a) => a.len() as u32,
    }
}
