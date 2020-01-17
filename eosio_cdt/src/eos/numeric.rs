use bytes::{Buf, BufMut, Bytes, BytesMut};
use super::EosSerialize;

impl EosSerialize for u8 {
    fn read(buf: &mut Bytes) -> Self {
        buf.get_u8()
    }

    fn write(&self, buf: &mut BytesMut) {
        buf.put_u8(*self);
    }
}

impl EosSerialize for u16 {
    fn read(buf: &mut Bytes) -> Self {
        buf.get_u16_le()
    }

    fn write(&self, buf: &mut BytesMut) {
        buf.put_u16_le(*self);
    }
}

impl EosSerialize for u32 {
    fn read(buf: &mut Bytes) -> Self {
        buf.get_u32_le()
    }

    fn write(&self, buf: &mut BytesMut) {
        buf.put_u32_le(*self);
    }
}

impl EosSerialize for u64 {
    fn read(buf: &mut Bytes) -> Self {
        buf.get_u64_le()
    }

    fn write(&self, buf: &mut BytesMut) {
        buf.put_u64_le(*self);
    }
}

pub fn push_varuint32(buf: &mut BytesMut, v: u32) {
    let mut val: u64 = v as u64;
    loop {
        let mut b: u8 = (val & 0x7f) as u8;
        val >>= 7;
        b |= (if val > 0 { 1 } else { 0 }) << 7;
        buf.put_u8(b);
        if val == 0 {
            break;
        }
    }
}

pub fn read_varuint32(buf: &mut Bytes) -> Result<u32, &'static str> {
    let mut value: u32 = 0;
    let mut shift = 0;

    loop {
        let current_byte = buf.get_u8();
        value |= (current_byte & 0x7f) as u32;
        value = value << shift;

        if (current_byte & 0x80) == 0 {
            break;
        }

        shift += 7;
        if shift >= 35 {
            return Err("invalid varuint32 encoding");
        }
    }

    Ok(value)
}
