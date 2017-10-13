use bytes::{BytesMut, BigEndian, Buf};

use std::io::{Cursor, Seek, SeekFrom};

use frame::{Frame, FrameHeader, FrameType, FramePayload, FRAME_END_OCTET, method, content_body,
            content_header};

const FRAME_HEADER_BYTE_SIZE: usize = 7;

pub fn decode_frame(src: &mut BytesMut) -> Option<Frame> {

    debug!("Decode frame : {:?}", src);

    match extract_frame_bytes(src) {
        Some(mut frame_bytes) => {
            debug!("Extracted a frame : {:?}", frame_bytes);

            let (typ, channel, payload_size) = decode_header(frame_bytes.split_to(FRAME_HEADER_BYTE_SIZE));

            debug!("frame type is {:?}", typ);
            debug!("frame channel is {}", channel);
            debug!("frame payload_size is {}", payload_size);

            let payload = decode_payload(&typ, frame_bytes.split_to(payload_size as usize));

            let frame = Frame {
                header: FrameHeader {
                    channel: channel,
                    frame_type: typ,
                },
                payload: payload,
            };

            debug!("Finish decoding frame : {:?}", frame);

            Some(frame)
        }
        None => None,
    }
}


/// Extract a frame bytes.
/// If there is not enough length to make frame, this function returns None.
/// If there is enough length, this function extract it after check frame end.
///
/// # Panics
/// If frame end is invalid
fn extract_frame_bytes(src: &mut BytesMut) -> Option<BytesMut> {
    if src.len() < 8 {
        None
    } else {
        let mut cursor = Cursor::new(src);
        cursor.seek(SeekFrom::Current(3_i64)).expect("Never fail");
        let size = cursor.get_u32::<BigEndian>() as usize;

        let src = cursor.into_inner();

        if src.len() >= size + 8 {
            let bytes = src.split_to(size + 8);

            // Check frame end
            if !bytes.as_ref().ends_with(&[FRAME_END_OCTET]) {
                panic!("Invalid Frame End");
            }

            Some(bytes)
        } else {
            None
        }
    }
}


/// Decode frame header. This function returns tuple of (type_octet, channel_id, body_size).
///
/// # Panics
/// when `src` does not have enough length.
fn decode_header(bytes: BytesMut) -> (FrameType, u16, u32) {
    let mut cursor = Cursor::new(bytes);
    let typ = match cursor.get_u8() {
        1 => FrameType::Method,
        2 => FrameType::ContentHeader,
        3 => FrameType::ContentBody,
        4 | 8 => FrameType::Heartbeat, // RabbitMQ sends heartbeat frame starting with 8
        b => unreachable!("Unexpected frame type '{}' is received", b),
    };
    let channel = cursor.get_u16::<BigEndian>();
    let size = cursor.get_u32::<BigEndian>();

    (typ, channel, size)
}


/// Decode frame payload with `FrameType`.
/// You **MUTS** gime `Bytes` which has **EXACT* length of payload (without frame-end).
///
/// # Panics
/// when `payload` does not have enough length.
fn decode_payload(typ: &FrameType, payload: BytesMut) -> FramePayload {
    use self::FrameType::*;
    let payload = match *typ {
        Method => FramePayload::Method(method::decoder::decode_payload(payload)),
        ContentHeader => FramePayload::ContentHeader(content_header::decode_payload(payload)),
        ContentBody => FramePayload::ContentBody(content_body::decode_payload(payload)),
        Heartbeat => FramePayload::Heartbeat,
    };
    payload
}
