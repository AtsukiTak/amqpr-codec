//! # Reference
//! https://www.rabbitmq.com/amqp-0-9-1-reference.html
//! Above link tells you general and rabbitmq-specific classes and methods.
//!
//! https://www.rabbitmq.com/amqp-0-9-1-quickref.html#exchange.bind
//! Above link tells you which method or field is rabbitmq-specific extension.
//!
//! https://www.rabbitmq.com/extensions.html
//! Above link tells you rabbitmq-specific extension.
//!
//! https://www.rabbitmq.com/resources/specs/amqp0-9-1.extended.xml
//! Above link tells you method id of rabbitmq-specific extension method.

pub mod decoder;
pub mod encoder;

use std::collections::HashMap;

use args::FieldArgument;

pub use self::connection::ConnectionClass;
pub use self::channel::ChannelClass;
pub use self::exchange::ExchangeClass;
pub use self::queue::QueueClass;
pub use self::basic::BasicClass;
pub use self::tx::TxClass;


#[derive(PartialEq, Clone, Debug)]
pub enum MethodPayload {
    Connection(ConnectionClass),
    Channel(ChannelClass),
    Exchange(ExchangeClass),
    Queue(QueueClass),
    Basic(BasicClass),
    Tx(TxClass),
}



// Connection module  {{{
pub mod connection {
    use super::*;

    #[derive(PartialEq, Clone, Debug)]
    pub enum ConnectionClass {
        Start(StartMethod),
        StartOk(StartOkMethod),
        Secure(SecureMethod),
        SecureOk(SecureOkMethod),
        Tune(TuneMethod),
        TuneOk(TuneOkMethod),
        Open(OpenMethod),
        OpenOk(OpenOkMethod),
        Close(CloseMethod),
        CloseOk,

        // These are rabbitmq-specific extensions
        Blocked(BlockedMethod),
        Unblocked,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct StartMethod {
        pub version_major: u8,
        pub version_minor: u8,
        pub server_properties: HashMap<String, FieldArgument>,
        pub mechanisms: String,
        pub locales: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct StartOkMethod {
        pub client_properties: HashMap<String, FieldArgument>,
        pub mechanism: String,
        pub response: String,
        pub locale: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct SecureMethod {
        pub challenge: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct SecureOkMethod {
        pub response: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct TuneMethod {
        pub channel_max: u16,
        pub frame_max: u32,
        pub heartbeat: u16,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct TuneOkMethod {
        pub channel_max: u16,
        pub frame_max: u32,
        pub heartbeat: u16,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct OpenMethod {
        pub virtual_host: String,
        pub reserved1: String,
        pub reserved2: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct OpenOkMethod {
        pub reserved1: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct CloseMethod {
        pub reply_code: u16,
        pub reply_text: String,
        pub class_id: u16,
        pub method_id: u16,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct BlockedMethod {
        pub reason: String,
    }
}
// }}}


// Channel module {{{
pub mod channel {
    #[derive(PartialEq, Clone, Debug)]
    pub enum ChannelClass {
        Open(OpenMethod),
        OpenOk(OpenOkMethod),
        Flow(FlowMethod),
        FlowOk(FlowOkMethod),
        Close(CloseMethod),
        CloseOk,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct OpenMethod {
        pub reserved1: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct OpenOkMethod {
        pub reserved1: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct FlowMethod {
        pub active: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct FlowOkMethod {
        pub active: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct CloseMethod {
        pub reply_code: u16,
        pub reply_text: String,
        pub class_id: u16,
        pub method_id: u16,
    }
}
// }}}


// Exchange module {{{
pub mod exchange {
    use super::*;

    #[derive(PartialEq, Clone, Debug)]
    pub enum ExchangeClass {
        Declare(DeclareMethod),
        DeclareOk,
        Delete(DeleteMethod),
        DeleteOk,
        Bind(BindMethod),
        BindOk, // rabbitmq-specific extension
        Unbind(UnbindMethod),
        UnbindOk, // rabbitmq-specific extension
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct DeclareMethod {
        pub reserved1: u16,
        pub exchange: String,
        pub typ: String,
        pub passive: bool,
        pub durable: bool,
        pub auto_delete: bool, // rabbitmq-specific extension. In another implementation, this is reserved.
        pub internal: bool, // rabbitmq-specific extension. In another implementation, this is reserved.
        pub no_wait: bool,
        pub arguments: HashMap<String, FieldArgument>,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct DeleteMethod {
        pub reserved1: u16,
        pub exchange: String,
        pub if_unused: bool,
        pub no_wait: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct BindMethod {
        pub reserved1: u16,
        pub destination: String,
        pub source: String,
        pub routing_key: String,
        pub no_wait: bool,
        pub arguments: HashMap<String, FieldArgument>,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct UnbindMethod {
        pub reserved1: u16,
        pub destination: String,
        pub source: String,
        pub routing_key: String,
        pub no_wait: bool,
        pub arguments: HashMap<String, FieldArgument>,
    }
}
// }}}


// Queue module {{{
pub mod queue {
    use super::*;

    #[derive(PartialEq, Clone, Debug)]
    pub enum QueueClass {
        Declare(DeclareMethod),
        DeclareOk(DeclareOkMethod),
        Bind(BindMethod),
        BindOk,
        Unbind(UnbindMethod),
        UnbindOk,
        Purge(PurgeMethod),
        PurgeOk(PurgeOkMethod),
        Delete(DeleteMethod),
        DeleteOk(DeleteOkMethod),
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct DeclareMethod {
        pub reserved1: u16,
        pub queue: String,
        pub passive: bool,
        pub durable: bool,
        pub exclusive: bool,
        pub auto_delete: bool,
        pub no_wait: bool,
        pub arguments: HashMap<String, FieldArgument>,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct DeclareOkMethod {
        pub queue: String,
        pub message_count: u32,
        pub consumer_count: u32,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct BindMethod {
        pub reserved1: u16,
        pub queue: String,
        pub exchange: String,
        pub routing_key: String,
        pub no_wait: bool,
        pub arguments: HashMap<String, FieldArgument>,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct UnbindMethod {
        pub reserved1: u16,
        pub queue: String,
        pub exchange: String,
        pub routing_key: String,
        pub arguments: HashMap<String, FieldArgument>,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct PurgeMethod {
        pub reserved1: u16,
        pub queue: String,
        pub no_wait: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct PurgeOkMethod {
        pub message_count: u32,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct DeleteMethod {
        pub reserved1: u16,
        pub queue: String,
        pub if_unused: bool,
        pub if_empty: bool,
        pub no_wait: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct DeleteOkMethod {
        pub message_count: u32,
    }
}
// }}}


// Basic module {{{
pub mod basic {
    use super::*;

    /// # Sent by client ( need to be encoded )
    /// - Qos
    /// - Consume
    /// - Cancel
    /// - Publish
    /// - Get
    /// - Ack
    /// - Reject
    /// - Nack
    /// - RecoverAsync
    /// - Recover
    ///
    /// # Received by client ( need to be decoded )
    /// - QosOk
    /// - ConsumeOk
    /// - CancelOk
    /// - Return
    /// - Deliver
    /// - Ack
    /// - Nack
    /// - RecoverOk
    #[derive(PartialEq, Clone, Debug)]
    pub enum BasicClass {
        Qos(QosMethod),
        QosOk,
        Consume(ConsumeMethod),
        ConsumeOk(ConsumeOkMethod),
        Cancel(CancelMethod),
        CancelOk(CancelOkMethod),
        Publish(PublishMethod),
        Return(ReturnMethod),
        Deliver(DeliverMethod),
        Get(GetMethod),
        GetOk(GetOkMethod),
        GetEmpty(GetEmptyMethod),
        Ack(AckMethod),
        Reject(RejectMethod),
        Nack(NackMethod), // rabbitmq-specific extension
        RecoverAsync(RecoverAsyncMethod),
        Recover(RecoverMethod),
        RecoverOk,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct QosMethod {
        pub prefetch_size: u32,
        pub prefetch_count: u16,
        pub global: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct ConsumeMethod {
        pub reserved1: u16,
        pub queue: String,
        pub consumer_tag: String,
        pub no_local: bool,
        pub no_ack: bool,
        pub exclusive: bool,
        pub no_wait: bool,
        pub arguments: HashMap<String, FieldArgument>,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct ConsumeOkMethod {
        pub consumer_tag: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct CancelMethod {
        pub consumer_tag: String,
        pub no_wait: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct CancelOkMethod {
        pub consumer_tag: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct PublishMethod {
        pub reserved1: u16,
        pub exchange: String,
        pub routing_key: String,
        pub mandatory: bool,
        pub immediate: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct ReturnMethod {
        pub reply_code: u16,
        pub reply_text: String,
        pub exchange: String,
        pub routing_key: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct DeliverMethod {
        pub consumer_tag: String,
        pub delivery_tag: u64,
        pub redeliverd: bool,
        pub exchange: String,
        pub routing_key: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct GetMethod {
        pub reserved1: u16,
        pub queue: String,
        pub no_ack: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct GetOkMethod {
        pub delivery_tag: u64,
        pub redeliverd: bool,
        pub exchange: String,
        pub routing_key: String,
        pub message_count: u32,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct GetEmptyMethod {
        pub reserved1: String,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct AckMethod {
        pub delivery_tag: u64,
        pub multiple: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct RejectMethod {
        pub delivery_tag: u64,
        pub requeue: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct RecoverAsyncMethod {
        pub requeue: bool,
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct RecoverMethod {
        pub requeue: bool,
    }

    // rabbitmq-specific extension
    #[derive(PartialEq, Clone, Debug)]
    pub struct NackMethod {
        pub delivery_tag: u64,
        pub multiple: bool,
    }
}
// }}}


// Tx module {{{
pub mod tx {
    /// # Sent by client ( need to be encoded )
    /// - Select
    /// - Commit
    /// - Rollback
    ///
    /// # Receive by client ( need to be decoded )
    /// - SelectOk
    /// - CommitOk
    /// - RollbackOk
    #[derive(PartialEq, Clone, Debug)]
    pub enum TxClass {
        Select,
        SelectOk,
        Commit,
        CommitOk,
        Rollback,
        RollbackOk,
    }
}
// }}}



// Tests {{{
#[cfg(test)]
mod tests {
    use args::*;
    use frame::method::decoder::decode_argument_with_info;
    use frame::method::encoder::encode_argument;
    use std::io::Cursor;

    fn decode_and_encode_argument(arg: Argument) {
        let mut dst = Vec::new();
        let info = decoding_info_from_arg(&arg);
        encode_argument(&arg, &mut dst).unwrap();
        assert_eq!(
            arg,
            decode_argument_with_info(&mut Cursor::new(dst.into()), &info).unwrap()
        );
    }


    fn decoding_info_from_arg(arg: &Argument) -> ArgumentDecodingInfo {
        match arg {
            &Argument::Octet(_) => ArgumentDecodingInfo::Octet,
            &Argument::Short(_) => ArgumentDecodingInfo::Short,
            &Argument::Long(_) => ArgumentDecodingInfo::Long,
            &Argument::LongLong(_) => ArgumentDecodingInfo::LongLong,
            &Argument::Bits(_) => ArgumentDecodingInfo::Bits,
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
        decode_and_encode_argument(Argument::ShortString(
            ShortString("HOGEHOGE Testing".into()),
        ));
        decode_and_encode_argument(Argument::LongString(LongString("FUGAFUGA Testing".into())));
    }

    #[test]
    fn decode_and_encode_field_table() {
        let long_str = FieldArgument::LongString(LongString("LongString HOGE!!!".into()));
        let short_str = FieldArgument::ShortString(ShortString("ShortString HOGE!!!".into()));

        let field_table = FieldTable(vec![
            ("long string".into(), long_str),
            ("short string".into(), short_str),
        ]);
        decode_and_encode_argument(Argument::FieldTable(field_table));
    }

    #[test]
    fn decode_and_encode_nested_field_table() {
        let long_str = FieldArgument::LongString(LongString("LongString HOGE!!!".into()));
        let short_str = FieldArgument::ShortString(ShortString("ShortString HOGE!!!".into()));
        let inner_field_table = FieldTable(vec![
            ("long string".into(), long_str.clone()),
            ("short string".into(), short_str.clone()),
        ]);

        let outer_field_table = FieldTable(vec![
            (
                "field table".into(),
                FieldArgument::NestedTable(inner_field_table)
            ),
            ("outer long string".into(), long_str),
            ("outer short string".into(), short_str),
        ]);

        decode_and_encode_argument(Argument::FieldTable(outer_field_table));
    }
}
// }}}
