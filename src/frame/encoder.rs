use bytes::{BufMut, BytesMut, BigEndian};

use frame::*;


pub fn encode_frame(item: Frame, dst: &mut BytesMut) -> Result<()> {
    debug!("Start encode frame : {:?}", item);

    // payloads
    let payload_bytes = {
        let mut buf = Vec::new();
        match item.payload {
            FramePayload::Method(payload) => method::encoder::encode_payload(payload, &mut buf)?,
            FramePayload::ContentHeader(payload) => {
                content_header::encode_payload(payload, &mut buf)?
            }
            FramePayload::ContentBody(payload) => {
                content_body::encoder::encode_payload(payload, &mut buf)?
            }
            FramePayload::Heartbeat => (),
        };
        buf
    };

    // put headers
    dst.reserve(7);
    dst.put_u8(frame_type_into_byte(&item.header.frame_type));
    dst.put_u16::<BigEndian>(item.header.channel);
    dst.put_u32::<BigEndian>(payload_bytes.len() as u32);
    debug!("byte size of payload is {}", payload_bytes.len());

    // put payload
    dst.extend_from_slice(payload_bytes.as_ref());

    // put frame-end
    dst.reserve(1);
    dst.put_u8(FRAME_END_OCTET);

    Ok(())
}


fn frame_type_into_byte(typ: &FrameType) -> u8 {
    match typ {
        &FrameType::Method => 1,
        &FrameType::ContentHeader => 2,
        &FrameType::ContentBody => 3,
        &FrameType::Heartbeat => 8, // For RabbitMQ, Qpid and OpenAMQ
    }
}
