use bytes::{Bytes, BytesMut, BigEndian, Buf};

use std::io::{Cursor, Seek, SeekFrom};

use frame::{Frame, FrameHeader, FrameType, FramePayload, FRAME_END_OCTET, method, content_body,
            content_header};
use errors::*;

pub fn decode_frame(src: &mut BytesMut) -> Result<Option<Frame>> {

    debug!("Decode frame : {:?}", src);

    match extract_frame_bytes(src)? {
        Some(frame_bytes) => {
            debug!("Extracted a frame : {:?}", frame_bytes);

            let mut cursor = Cursor::new(frame_bytes);

            let (typ, channel, payload_size) = decode_header(&mut cursor)?;

            debug!("frame type is {:?}", typ);
            debug!("frame channel is {}", channel);
            debug!("frame payload_size is {}", payload_size);

            let payload = decode_payload(&typ, &mut cursor)?;

            // return Err if frame-end is invlaid.
            check_frame_end(&mut cursor)?;

            let frame = Frame {
                header: FrameHeader {
                    channel: channel,
                    frame_type: typ,
                },
                payload: payload,
            };

            debug!("Finish decoding frame : {:?}", frame);

            Ok(Some(frame))
        }
        None => Ok(None),
    }
}


/// Extract a frame bytes.
/// If there is not enough length to make frame, this function returns None.
/// If there is enough length, this function extract it.
fn extract_frame_bytes(src: &mut BytesMut) -> Result<Option<Bytes>> {
    if src.len() < 8 {
        Ok(None)
    } else {
        let mut cursor = Cursor::new(src);
        cursor.seek(SeekFrom::Current(3 as i64))?;
        let size = cursor.get_u32::<BigEndian>() as usize;

        let src = cursor.into_inner();

        if src.len() >= size + 8 {
            Ok(Some(src.split_to(size + 8).freeze()))
        } else {
            Ok(None)
        }
    }
}


/// Decode frame header. This function returns tuple of (type_octet, channel_id, body_size).
/// You **MUST** give `Bytes` which has enough length.
///
/// # Panics
/// when `src` does not have enough length.
fn decode_header(cursor: &mut Cursor<Bytes>) -> Result<(FrameType, u16, u32)> {
    let typ = match cursor.get_u8() {
        1 => FrameType::Method,
        2 => FrameType::ContentHeader,
        3 => FrameType::ContentBody,
        4 | 8 => FrameType::Heartbeat, // RabbitMQ sends heartbeat frame starting with 8
        b => return Err(ErrorKind::InvalidFrameTypeByte(b).into()),
    };
    let channel = cursor.get_u16::<BigEndian>();
    let size = cursor.get_u32::<BigEndian>();

    Ok((typ, channel, size))
}


/// Check frame-end.
/// We do it before read payload because
/// we can find below text in the official document.
///
/// """ When a peer reads a frame it MUST check that the frame-end is valud before
/// attempting to decode the frame."""
///
/// TODO
/// Investigate above restriction is needed. If not, we can improve performance.
fn check_frame_end(cursor: &mut Cursor<Bytes>) -> Result<()> {
    if cursor.get_ref().ends_with(&[FRAME_END_OCTET]) {
        Ok(())
    } else {
        Err(ErrorKind::InvalidFrameEnd.into())
    }
}


/// Decode frame payload with `FrameType`.
/// You **MUTS** gime `Bytes` which has **EXACT* length of payload (without frame-end).
///
/// # Panics
/// when `payload` does not have enough length.
fn decode_payload(typ: &FrameType, payload: &mut Cursor<Bytes>) -> Result<FramePayload> {
    use self::FrameType::*;
    let payload = match *typ {
        Method => FramePayload::Method(method::decoder::decode_payload(payload)?),
        ContentHeader => FramePayload::ContentHeader(content_header::decode_payload(payload)?),
        ContentBody => FramePayload::ContentBody(content_body::decode_payload(payload)?),
        Heartbeat => FramePayload::Heartbeat,
    };
    Ok(payload)
}
