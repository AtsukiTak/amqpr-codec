use std::collections::HashMap;
use std::ops::Deref;
use std::convert::From;

use bytes::Bytes;


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
    ShortString(AmqpString),
    LongString(AmqpString),
    // Array(), // I can not find any definition of this field.
    Timestamp(u64),
    NestedTable(HashMap<AmqpString, FieldArgument>),
    Void,
    ByteArray(Vec<u8>), // How can we treat it
}



/// String being able to do Zero-cost conversion from `Bytes` or `&'static str`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AmqpString(pub(crate) Bytes);


impl Deref for AmqpString {
    type Target = str;

    fn deref(&self) -> &str {
        ::std::str::from_utf8(self.0.as_ref()).expect("Non Utf-8 bytes")
    }
}


impl From<Vec<u8>> for AmqpString {
    fn from(bytes: Vec<u8>) -> AmqpString {
        AmqpString(Bytes::from(bytes))
    }
}

impl From<String> for AmqpString {
    fn from(bytes: String) -> AmqpString {
        AmqpString(Bytes::from(bytes))
    }
}

impl<'a> From<&'a [u8]> for AmqpString {
    fn from(bytes: &'a [u8]) -> AmqpString {
        AmqpString(Bytes::from(bytes))
    }
}

impl From<&'static str> for AmqpString {
    fn from(bytes: &'static str) -> AmqpString {
        AmqpString(Bytes::from_static(bytes.as_bytes()))
    }
}
