pub mod decoder;
pub mod encoder;

pub use self::encoder::byte_size_of_payload;

use methods::args::Arguments;

#[derive(PartialEq, Clone, Debug)]
pub struct MethodPayload {
    pub class_id: u16,
    pub method_id: u16,
    pub arguments: Arguments,
}


#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use methods::args::*;
    use frame::method::decoder::decode_argument_with_info;
    use frame::method::encoder::encode_argument;
    use std::io::Cursor;

    fn decode_and_encode_argument(arg: Argument) {
        let mut dst = BytesMut::new();
        let info = decoding_info_from_arg(&arg);
        encode_argument(&arg, &mut dst).unwrap();
        assert_eq!(arg,
                   decode_argument_with_info(&mut Cursor::new(dst.freeze()), &info).unwrap());
    }


    fn decoding_info_from_arg(arg: &Argument) -> ArgumentDecodingInfo {
        match arg {
            &Argument::Octet(_) => ArgumentDecodingInfo::Octet,
            &Argument::Short(_) => ArgumentDecodingInfo::Short,
            &Argument::Long(_) => ArgumentDecodingInfo::Long,
            &Argument::LongLong(_) => ArgumentDecodingInfo::LongLong,
            &Argument::Bits(Bits(ref vec)) => ArgumentDecodingInfo::Bits(vec.len()),
            &Argument::ShortString(_) => ArgumentDecodingInfo::ShortString,
            &Argument::LongString(_) => ArgumentDecodingInfo::LongString,
            &Argument::Timestamp(_) => ArgumentDecodingInfo::Timestamp,
            &Argument::FieldTable(_) => ArgumentDecodingInfo::FieldTable,
            &Argument::FieldArray(_) => ArgumentDecodingInfo::FieldArray,
        }
    }

    #[test]
    fn decode_and_encode_simple_arguments() {
        decode_and_encode_argument(Argument::Octet(42));
        decode_and_encode_argument(Argument::Short(42));
        decode_and_encode_argument(Argument::Long(42));
        decode_and_encode_argument(Argument::LongLong(42));
        decode_and_encode_argument(Argument::ShortString(ShortString("HOGEHOGE Testing".into())));
        decode_and_encode_argument(Argument::LongString(LongString("FUGAFUGA Testing".into())));
    }

    #[test]
    fn decode_and_encode_field_table() {
        let long_str = FieldArgument::LongString(LongString("LongString HOGE!!!".into()));
        let short_str = FieldArgument::ShortString(ShortString("ShortString HOGE!!!".into()));

        let field_table = FieldTable(vec![("long string".into(), long_str),
                                          ("short string".into(), short_str)]);
        decode_and_encode_argument(Argument::FieldTable(field_table));
    }

    #[test]
    fn decode_and_encode_nested_field_table() {
        let long_str = FieldArgument::LongString(LongString("LongString HOGE!!!".into()));
        let short_str = FieldArgument::ShortString(ShortString("ShortString HOGE!!!".into()));
        let inner_field_table = FieldTable(vec![("long string".into(), long_str.clone()),
                                                ("short string".into(), short_str.clone())]);

        let outer_field_table = FieldTable(vec![("field table".into(),
                                                 FieldArgument::NestedTable(inner_field_table)),
                                                ("outer long string".into(), long_str),
                                                ("outer short string".into(), short_str)]);

        decode_and_encode_argument(Argument::FieldTable(outer_field_table));
    }
}
