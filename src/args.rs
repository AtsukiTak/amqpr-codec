use std::collections::HashMap;


#[derive(PartialEq, Clone, Debug)]
pub enum FieldArgument {
    Boolean(bool),
    SignedOctet(i8),
    UnsignedOctet(u8),
    SignedShort(i16),
    UnsignedShort(u16),
    SignedLong(i32),
    UnsignedLong(u32),
    SignedLongLong(i64),
    UnsignedLongLong(u64),
    Float(f32),
    Double(f64),
    Decimal(i64), // For now, we do not handle big number which is bigger than max of i64.
    ShortString(String),
    LongString(String),
    // Array(), // I can not find any definition of this field.
    Timestamp(u64),
    NestedTable(HashMap<String, FieldArgument>),
    Void,
    ByteArray(Vec<u8>), // How can we treat it
}
