use bytes::{BufMut, BigEndian};

use super::MethodPayload;
use methods::args::*;
use errors::*;

pub fn encode_payload(payload: MethodPayload, dst: &mut Vec<u8>) -> Result<()> {
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
            dst.put_u8(byte);
        }
        &Argument::Short(short) => {
            dst.put_u16::<BigEndian>(short);
        }
        &Argument::Long(long) => {
            dst.put_u32::<BigEndian>(long);
        }
        &Argument::LongLong(longlong) => {
            dst.put_u64::<BigEndian>(longlong);
        }
        &Argument::Bits(ref byte) => {
            dst.put_u8(byte.clone());
        }
        &Argument::ShortString(ref string) => {
            dst.put_u8(string.len() as u8);
            dst.put(&string.0);
        }
        &Argument::LongString(ref string) => {
            dst.put_u32::<BigEndian>(string.len() as u32);
            dst.put(&string.0);
        }
        &Argument::Timestamp(Timestamp(t)) => {
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
    let mut bytes = {
        let mut buf = Vec::new();
        for &(ref item_name, ref item_value) in table.iter() {
            buf.put_u8(item_name.len() as u8);
            buf.put(item_name);
            encode_field_item(item_value, &mut buf)?;
        }
        buf
    };

    dst.put_u32::<BigEndian>(bytes.len() as u32);
    dst.append(&mut bytes);
    Ok(())
}


fn encode_field_array(array: &FieldArray, dst: &mut Vec<u8>) -> Result<()> {
    let mut bytes = {
        let mut buf = Vec::new();
        for item in array.iter() {
            encode_field_item(item, &mut buf)?;
        }
        buf
    };

    dst.put_u32::<BigEndian>(bytes.len() as u32);
    dst.append(&mut bytes);
    Ok(())
}


fn encode_field_item(item: &FieldArgument, dst: &mut Vec<u8>) -> Result<()> {
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
        &FieldArgument::ShortString(ref sstr) => {
            dst.put_u8(b's');
            dst.put_u8(sstr.len() as u8);
            dst.put(&sstr.0);
        }
        &FieldArgument::LongString(ref lstr) => {
            dst.put_u8(b'S');
            dst.put_u32::<BigEndian>(lstr.len() as u32);
            dst.put(&lstr.0);
        }
        &FieldArgument::Timestamp(ts) => {
            dst.put_u8(b'T');
            dst.put_u64::<BigEndian>(ts);
        }
        &FieldArgument::NestedTable(ref table) => {
            dst.put_u8(b'F');
            encode_field_table(table, dst)?;
        }
        &FieldArgument::Void => {
            dst.put_u8(b'V');
        }
        &FieldArgument::ByteArray(ref _array) => {
            dst.put_u8(b'x');
            panic!("Fail to parse ByteArray") // I don't know how should I treat it
        }
    }
    Ok(())
}
