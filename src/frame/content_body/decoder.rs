use bytes::BytesMut;

use super::ContentBodyPayload;

pub fn decode_payload(payload: &mut BytesMut) -> ContentBodyPayload {
    ContentBodyPayload { bytes: payload.clone().freeze() }
}
