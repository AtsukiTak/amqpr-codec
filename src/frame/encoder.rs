use bytes::{BufMut, BigEndian};

use frame::*;


pub fn encode_frame(item: Frame, dst: &mut Vec<u8>) -> Result<()> {
    debug!("Start encode frame : {:?}", item);

    // reserve for frame header and frame-end.
    // dst.reserve(8);

    // put headers
    dst.put_u8(frame_type_into_byte(&item.header.frame_type));
    dst.put_u16::<BigEndian>(item.header.channel);
    dst.put_u32::<BigEndian>(byte_size_of_payload(&item.payload));
    debug!("byte size of payload is {}",
           byte_size_of_payload(&item.payload));

    // put payloads
    match item.payload {
        FramePayload::Method(payload) => method::encoder::encode_payload(payload, dst)?, 
        FramePayload::ContentHeader(payload) => content_header::encode_payload(payload, dst)?, 
        FramePayload::ContentBody(payload) => content_body::encoder::encode_payload(payload, dst)?, 
        FramePayload::Heartbeat => (),
    };

    // put frame-end
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



fn byte_size_of_payload(payload: &FramePayload) -> u32 {
    match payload {
        &FramePayload::Method(ref payload) => method::byte_size_of_payload(payload),
        &FramePayload::ContentHeader(ref payload) => content_header::byte_size_of_payload(payload),
        &FramePayload::ContentBody(ref payload) => {
            content_body::encoder::byte_size_of_payload(payload)
        }
        &FramePayload::Heartbeat => 0u32,
    }
}
