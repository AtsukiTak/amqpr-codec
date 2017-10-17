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


// Implementation of MethodPayload {{{
impl MethodPayload {
    pub fn connection(&self) -> Option<&ConnectionClass> {
        match self {
            &MethodPayload::Connection(ref c) => Some(c),
            _ => None,
        }
    }

    pub fn channel(&self) -> Option<&ChannelClass> {
        match self {
            &MethodPayload::Channel(ref c) => Some(c),
            _ => None,
        }
    }

    pub fn exchange(&self) -> Option<&ExchangeClass> {
        match self {
            &MethodPayload::Exchange(ref c) => Some(c),
            _ => None,
        }
    }

    pub fn queue(&self) -> Option<&QueueClass> {
        match self {
            &MethodPayload::Queue(ref c) => Some(c),
            _ => None,
        }
    }

    pub fn basic(&self) -> Option<&BasicClass> {
        match self {
            &MethodPayload::Basic(ref c) => Some(c),
            _ => None,
        }
    }

    pub fn tx(&self) -> Option<&TxClass> {
        match self {
            &MethodPayload::Tx(ref c) => Some(c),
            _ => None,
        }
    }
}
// }}}



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

    // Implementation of ConnectionClass {{{
    impl ConnectionClass {
        pub fn start(&self) -> Option<&StartMethod> {
            match self {
                &ConnectionClass::Start(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn start_ok(&self) -> Option<&StartOkMethod> {
            match self {
                &ConnectionClass::StartOk(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn secure(&self) -> Option<&SecureMethod> {
            match self {
                &ConnectionClass::Secure(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn secure_ok(&self) -> Option<&SecureOkMethod> {
            match self {
                &ConnectionClass::SecureOk(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn tune(&self) -> Option<&TuneMethod> {
            match self {
                &ConnectionClass::Tune(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn tune_ok(&self) -> Option<&TuneOkMethod> {
            match self {
                &ConnectionClass::TuneOk(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn open(&self) -> Option<&OpenMethod> {
            match self {
                &ConnectionClass::Open(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn open_ok(&self) -> Option<&OpenOkMethod> {
            match self {
                &ConnectionClass::OpenOk(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn close(&self) -> Option<&CloseMethod> {
            match self {
                &ConnectionClass::Close(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn close_ok(&self) -> Option<()> {
            match self {
                &ConnectionClass::CloseOk => Some(()),
                _ => None,
            }
        }

        pub fn blocked(&self) -> Option<&BlockedMethod> {
            match self {
                &ConnectionClass::Blocked(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn unblocked(&self) -> Option<()> {
            match self {
                &ConnectionClass::Unblocked => Some(()),
                _ => None,
            }
        }
    }
    // }}}

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

    // Implementation of ChannelClass {{{
    impl ChannelClass {
        pub fn open(&self) -> Option<&OpenMethod> {
            match self {
                &ChannelClass::Open(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn open_ok(&self) -> Option<&OpenOkMethod> {
            match self {
                &ChannelClass::OpenOk(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn flow(&self) -> Option<&FlowMethod> {
            match self {
                &ChannelClass::Flow(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn flow_ok(&self) -> Option<&FlowOkMethod> {
            match self {
                &ChannelClass::FlowOk(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn close(&self) -> Option<&CloseMethod> {
            match self {
                &ChannelClass::Close(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn close_ok(&self) -> Option<()> {
            match self {
                &ChannelClass::CloseOk => Some(()),
                _ => None,
            }
        }
    }
    // }}}

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

    // Implementation of ExchangeClass {{{
    impl ExchangeClass {
        pub fn declare(&self) -> Option<&DeclareMethod> {
            match self {
                &ExchangeClass::Declare(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn declare_ok(&self) -> Option<()> {
            match self {
                &ExchangeClass::DeclareOk => Some(()),
                _ => None,
            }
        }

        pub fn delete(&self) -> Option<&DeleteMethod> {
            match self {
                &ExchangeClass::Delete(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn delete_ok(&self) -> Option<()> {
            match self {
                &ExchangeClass::DeleteOk => Some(()),
                _ => None,
            }
        }

        pub fn bind(&self) -> Option<&BindMethod> {
            match self {
                &ExchangeClass::Bind(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn bind_ok(&self) -> Option<()> {
            match self {
                &ExchangeClass::BindOk => Some(()),
                _ => None,
            }
        }

        pub fn unbind(&self) -> Option<&UnbindMethod> {
            match self {
                &ExchangeClass::Unbind(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn unbind_ok(&self) -> Option<()> {
            match self {
                &ExchangeClass::UnbindOk => Some(()),
                _ => None,
            }
        }
    }
    // }}}

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

    // Implementation of QueueClass {{{
    impl QueueClass {
        pub fn declare(&self) -> Option<&DeclareMethod> {
            match self {
                &QueueClass::Declare(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn declare_ok(&self) -> Option<&DeclareOkMethod> {
            match self {
                &QueueClass::DeclareOk(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn bind(&self) -> Option<&BindMethod> {
            match self {
                &QueueClass::Bind(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn bind_ok(&self) -> Option<()> {
            match self {
                &QueueClass::BindOk => Some(()),
                _ => None,
            }
        }

        pub fn unbind(&self) -> Option<&UnbindMethod> {
            match self {
                &QueueClass::Unbind(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn unbind_ok(&self) -> Option<()> {
            match self {
                &QueueClass::UnbindOk => Some(()),
                _ => None,
            }
        }

        pub fn purge(&self) -> Option<&PurgeMethod> {
            match self {
                &QueueClass::Purge(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn purge_ok(&self) -> Option<&PurgeOkMethod> {
            match self {
                &QueueClass::PurgeOk(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn delete(&self) -> Option<&DeleteMethod> {
            match self {
                &QueueClass::Delete(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn delete_ok(&self) -> Option<&DeleteOkMethod> {
            match self {
                &QueueClass::DeleteOk(ref m) => Some(m),
                _ => None,
            }
        }
    }
    // }}}


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

    // Implementation of BasicClass {{{
    impl BasicClass {
        pub fn qos(&self) -> Option<&QosMethod> {
            match self {
                &BasicClass::Qos(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn qos_ok(&self) -> Option<()> {
            match self {
                &BasicClass::QosOk => Some(()),
                _ => None,
            }
        }

        pub fn consume(&self) -> Option<&ConsumeMethod> {
            match self {
                &BasicClass::Consume(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn consume_ok(&self) -> Option<&ConsumeOkMethod> {
            match self {
                &BasicClass::ConsumeOk(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn cancel(&self) -> Option<&CancelMethod> {
            match self {
                &BasicClass::Cancel(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn cancel_ok(&self) -> Option<&CancelOkMethod> {
            match self {
                &BasicClass::CancelOk(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn publish(&self) -> Option<&PublishMethod> {
            match self {
                &BasicClass::Publish(ref m) => Some(m),
                _ => None,
            }
        }

        // We can not use the name "return" as function name.
        pub fn return_(&self) -> Option<&ReturnMethod> {
            match self {
                &BasicClass::Return(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn deliver(&self) -> Option<&DeliverMethod> {
            match self {
                &BasicClass::Deliver(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn get(&self) -> Option<&GetMethod> {
            match self {
                &BasicClass::Get(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn get_ok(&self) -> Option<&GetOkMethod> {
            match self {
                &BasicClass::GetOk(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn get_empty(&self) -> Option<&GetEmptyMethod> {
            match self {
                &BasicClass::GetEmpty(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn ack(&self) -> Option<&AckMethod> {
            match self {
                &BasicClass::Ack(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn reject(&self) -> Option<&RejectMethod> {
            match self {
                &BasicClass::Reject(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn nack(&self) -> Option<&NackMethod> {
            match self {
                &BasicClass::Nack(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn recover_async(&self) -> Option<&RecoverAsyncMethod> {
            match self {
                &BasicClass::RecoverAsync(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn recover(&self) -> Option<&RecoverMethod> {
            match self {
                &BasicClass::Recover(ref m) => Some(m),
                _ => None,
            }
        }

        pub fn recover_ok(&self) -> Option<()> {
            match self {
                &BasicClass::RecoverOk => Some(()),
                _ => None,
            }
        }
    }
    // }}}

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

    // Implementation of TxClass {{{
    impl TxClass {
        pub fn select(&self) -> Option<()> {
            match self {
                &TxClass::Select => Some(()),
                _ => None,
            }
        }

        pub fn select_ok(&self) -> Option<()> {
            match self {
                &TxClass::SelectOk => Some(()),
                _ => None,
            }
        }

        pub fn commit(&self) -> Option<()> {
            match self {
                &TxClass::Commit => Some(()),
                _ => None,
            }
        }

        pub fn commit_ok(&self) -> Option<()> {
            match self {
                &TxClass::CommitOk => Some(()),
                _ => None,
            }
        }

        pub fn rollback(&self) -> Option<()> {
            match self {
                &TxClass::Rollback => Some(()),
                _ => None,
            }
        }

        pub fn rollback_ok(&self) -> Option<()> {
            match self {
                &TxClass::RollbackOk => Some(()),
                _ => None,
            }
        }
    }
    // }}}
}
// }}}
