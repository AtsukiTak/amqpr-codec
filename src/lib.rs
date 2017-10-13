//! This crate defines codec of AMQP protocol.
//!
//! # Byte architecture of general AMQP message frame
//!
//! position   0      1         3      7         size+7      size+8
//!            +------+---------+------+---------+-----------+
//!            | type | channel | size | payload | frame_end |
//!            +------+---------+------+---------+-----------+
//! length        1        2        4     size         1
//!
//! "payload" is defined for each frame type.
//!
extern crate tokio_io;

extern crate bytes;

#[macro_use]
extern crate log;

pub mod frame;
mod args;
pub use args::FieldArgument;


use frame::Frame;
use bytes::BytesMut;
use std::io::Error as IoError;

pub struct Codec;

impl tokio_io::codec::Decoder for Codec {
    type Item = Frame;
    type Error = IoError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, IoError> {
        Ok(frame::decoder::decode_frame(src))
    }
}

impl tokio_io::codec::Encoder for Codec {
    type Item = Frame;
    type Error = IoError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), IoError> {
        Ok(frame::encoder::encode_frame(item, dst))
    }
}
