use bytes::BufMut;

use super::ContentBodyPayload;
use errors::*;

pub fn encode_payload(payload: ContentBodyPayload, dst: &mut Vec<u8>) -> Result<()> {
    debug!("Start encoding content body : {:?}", payload);

    // dst.reserve(payload.bytes.len() + 1);
    dst.put(payload.bytes);
    Ok(())
}
