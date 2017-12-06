pub mod decoder;
pub mod encoder;

pub use self::decoder::decode_payload;
pub use self::encoder::encode_payload;

use args::{AmqpString, FieldArgument};

use std::collections::HashMap;


#[derive(PartialEq, Clone, Debug)]
pub struct ContentHeaderPayload {
    pub class_id: u16,
    pub body_size: u64,
    pub properties: Properties,
}


#[derive(PartialEq, Clone, Debug)]
pub struct Properties {
    pub content_type: Option<AmqpString>,
    pub content_encoding: Option<AmqpString>,
    pub headers: Option<HashMap<AmqpString, FieldArgument>>,
    pub delivery_mode: Option<u8>,
    pub priority: Option<u8>,
    pub correlation_id: Option<AmqpString>,
    pub reply_to: Option<AmqpString>,
    pub expiration: Option<AmqpString>,
    pub message_id: Option<AmqpString>,
    pub timestamp: Option<i64>,
    pub type_: Option<AmqpString>,
    pub user_id: Option<AmqpString>,
    pub app_id: Option<AmqpString>,
}

impl Properties {
    pub fn new() -> Properties {
        Properties {
            content_type: None,
            content_encoding: None,
            headers: None,
            delivery_mode: None,
            priority: None,
            correlation_id: None,
            reply_to: None,
            expiration: None,
            message_id: None,
            timestamp: None,
            type_: None,
            user_id: None,
            app_id: None,
        }
    }
}


// TESTS {{{
#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn encode_and_decode_without_properties() {
        let payload = ContentHeaderPayload {
            class_id: 42,
            body_size: 10000,
            properties: Properties::new(),
        };

        let cloned = payload.clone();

        let mut encoded = BytesMut::from(encode_payload(payload));

        let decoded = decode_payload(&mut encoded);

        assert_eq!(decoded, cloned);
        assert_eq!(encoded.len(), 0);
    }

    #[test]
    fn encode_and_decode_with_properties() {
        let properties = {
            let mut ps = Properties::new();
            ps.content_type = Some(AmqpString::from("application/text"));
            ps.priority = Some(42);
            ps
        };
        let payload = ContentHeaderPayload {
            class_id: 42,
            body_size: 10000,
            properties: properties,
        };

        let cloned = payload.clone();

        let mut encoded = BytesMut::from(encode_payload(payload));

        let decoded = decode_payload(&mut encoded);

        assert_eq!(decoded, cloned);
        assert_eq!(encoded.len(), 0);
    }

}
// }}}
