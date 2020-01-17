use bytes::{Buf, Bytes};

pub fn push_varuint32(bin: &mut Vec<u8>, v: u32) {
    let mut val: u64 = v as u64;
    loop {
        let mut b: u8 = (val & 0x7f) as u8;
        val >>= 7;
        b |= (if val > 0 { 1 } else { 0 }) << 7;
        bin.push(b);
        if val == 0 {
            break;
        }
    }
}

pub fn read_varuint32(bin: &mut Bytes) -> Result<u32, &'static str> {
    let mut value: u32 = 0;
    let mut shift = 0;

    loop {
        let current_byte = bin.get_u8();
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