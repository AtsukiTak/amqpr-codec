use std::ops::Deref;

// TODO
// Using something instead of `Vec`
#[derive(PartialEq, Clone, Debug)]
pub struct Arguments(pub Vec<Argument>);

impl Deref for Arguments {
    type Target = Vec<Argument>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}



#[derive(PartialEq, Clone, Debug)]
pub enum Argument {
    Octet(u8),
    Short(u16),
    Long(u32),
    LongLong(u64),
    Bits(u8),
    ShortString(ShortString),
    LongString(LongString),
    Timestamp(Timestamp),
    FieldTable(FieldTable),
    FieldArray(FieldArray),
}


#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ShortString(pub String);

impl Deref for ShortString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}


#[derive(Eq, PartialEq, Clone, Debug)]
pub struct LongString(pub String);

impl Deref for LongString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}



#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Timestamp(pub u64);



#[derive(PartialEq, Clone, Debug)]
pub struct FieldTable(pub Vec<(String, FieldArgument)>);

impl Deref for FieldTable {
    type Target = Vec<(String, FieldArgument)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


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
    Decimal(i64), // See https://www.rabbitmq.com/amqp-0-9-1-errata.html. I dont know `scale long-unit`.
    ShortString(ShortString),
    LongString(LongString),
    // Array(), // I can not find any definition of this field.
    Timestamp(u64),
    NestedTable(FieldTable),
    Void,
    ByteArray(Vec<u8>), // How can we treat it
}


#[derive(PartialEq, Clone, Debug)]
pub struct FieldArray(pub Vec<FieldArgument>);

impl Deref for FieldArray {
    type Target = Vec<FieldArgument>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}



/// `ArgumentSymbol` is used when specify argument type that you decode byte array into
pub enum ArgumentDecodingInfo {
    Octet,
    Short,
    Long,
    LongLong,
    Bits,
    ShortString,
    LongString,
    Timestamp,
    FieldTable,
    FieldArray,
}
