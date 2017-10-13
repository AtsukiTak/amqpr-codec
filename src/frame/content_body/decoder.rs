use bytes::BytesMut;

use super::ContentBodyPayload;

pub fn decode_payload(payload: BytesMut) -> ContentBodyPayload {
    ContentBodyPayload { bytes: payload.freeze() }
}
