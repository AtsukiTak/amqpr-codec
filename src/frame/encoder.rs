use bytes::{BufMut, BytesMut, BigEndian};

use frame::*;


pub fn encode_frame(item: Frame, dst: &mut BytesMut) {
    debug!("Start encode frame : {:?}", item);

    match item.payload {
        FramePayload::Method(payload) => {
            let payload = method::encoder::encode_payload(payload);
            put_frame_data(item.header, payload.as_ref(), dst);
        }
        FramePayload::ContentHeader(payload) => {
            let payload = content_header::encode_payload(payload);
            put_frame_data(item.header, payload.as_ref(), dst);
        }
        FramePayload::ContentBody(payload) => {
            let payload = content_body::encoder::encode_payload(payload);
            put_frame_data(item.header, payload.as_ref(), dst);
        }
        FramePayload::Heartbeat => (),
    };
}


fn put_frame_data(header: FrameHeader, encoded_payload: &[u8], dst: &mut BytesMut)
{
    // put headers
    dst.reserve(7);
    dst.put_u8(frame_type_into_byte(&header.frame_type));
    dst.put_u16::<BigEndian>(header.channel);
    dst.put_u32::<BigEndian>(encoded_payload.len() as u32);
    debug!("byte size of payload is {}", encoded_payload.len());

    // put payload
    dst.extend_from_slice(encoded_payload);

    // put frame-end
    dst.reserve(1);
    dst.put_u8(FRAME_END_OCTET);
}


fn frame_type_into_byte(typ: &FrameType) -> u8 {
    match typ {
        &FrameType::Method => 1,
        &FrameType::ContentHeader => 2,
        &FrameType::ContentBody => 3,
        &FrameType::Heartbeat => 8, // For RabbitMQ, Qpid and OpenAMQ
    }
}
