use bytes::Bytes;

use super::ContentBodyPayload;

pub fn encode_payload(payload: ContentBodyPayload) -> Bytes {
    debug!("Start encoding content body : {:?}", payload);

    payload.bytes
}
