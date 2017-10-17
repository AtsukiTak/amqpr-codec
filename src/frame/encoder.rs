use bytes::{BufMut, BytesMut, BigEndian};

use frame::*;


const METHOD_TYPE_BYTE: u8 = 1;
const CONTENT_BODY_TYPE_BYTE: u8 = 2;
const CONTENT_HEADER_TYPE_BYTE: u8 = 3;
const HEARTBEAT_TYPE_BYTE: u8 = 8;


pub fn encode_frame(item: Frame, dst: &mut BytesMut) {
    debug!("Start encode frame : {:?}", item);

    match item.payload {
        FramePayload::Method(payload) => {
            let payload = method::encoder::encode_payload(payload);
            encode_frame_inner(METHOD_TYPE_BYTE, item.header.channel, payload.as_ref(), dst);
        }
        FramePayload::ContentHeader(payload) => {
            let payload = content_header::encode_payload(payload);
            encode_frame_inner(
                CONTENT_HEADER_TYPE_BYTE,
                item.header.channel,
                payload.as_ref(),
                dst,
            );
        }
        FramePayload::ContentBody(payload) => {
            let payload = content_body::encoder::encode_payload(payload);
            encode_frame_inner(
                CONTENT_BODY_TYPE_BYTE,
                item.header.channel,
                payload.as_ref(),
                dst,
            );
        }
        FramePayload::Heartbeat => {
            encode_frame_inner(HEARTBEAT_TYPE_BYTE, item.header.channel, &[], dst);
        }
    };
}


fn encode_frame_inner(type_byte: u8, channel: u16, encoded_payload: &[u8], dst: &mut BytesMut) {
    // put headers
    dst.reserve(7);
    dst.put_u8(type_byte);
    dst.put_u16::<BigEndian>(channel);
    dst.put_u32::<BigEndian>(encoded_payload.len() as u32);
    debug!("byte size of payload is {}", encoded_payload.len());

    // put payload
    dst.extend_from_slice(encoded_payload);

    // put frame-end
    dst.reserve(1);
    dst.put_u8(FRAME_END_OCTET);
}
