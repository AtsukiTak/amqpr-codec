use bytes::Bytes;

use std::io::{Cursor, BufRead};

use errors::*;
use super::ContentBodyPayload;

pub fn decode_payload(payload: &mut Cursor<Bytes>) -> Result<ContentBodyPayload> {
    let mut bytes = Bytes::new();
    bytes.extend_from_slice(payload.fill_buf()?);
    let len = bytes.len();
    bytes.truncate(len - 1);
    Ok(ContentBodyPayload { bytes: bytes })
}
