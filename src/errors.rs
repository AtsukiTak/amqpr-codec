error_chain! {
    foreign_links {
        IoError(::std::io::Error);
        Utf8Error(::std::str::Utf8Error);
    }

    errors {
        ProtocolNegotiationFailedError {
            description("Protocol negotiation is failed")
            display("Protocol negotiation is failed")
        }
        InvalidFrameEnd {
            description("Frame end is invalid")
            display("Frame end is invalid")
        }
        InvalidFrameTypeByte(byte: u8) {
            description("Invalid frame type byte")
            display("Invalid frame type byte : {}", byte)
        }
        UnknownClassIdOrMethodId(class_id: u16, method_id: u16) {
            description("Unknown class id or method id")
            display("Unknown class id {} or method id {}", class_id, method_id)
        }
        InvalidFieldArgumentTypeByte(byte: u8) {
            description("Invalid argument type byte")
            display("Invalid argument type byte : {}", byte)
        }
    }
}
