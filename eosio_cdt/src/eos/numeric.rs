use bytes::{Buf, Bytes};
use serde::{Serialize, Deserialize, Serializer, Deserializer};

pub struct Varuint32(u32);

impl Serialize for Varuint32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes = Vec::new();
        let mut val: u64 = self.0 as u64;
        loop {
            let mut b: u8 = (val & 0x7f) as u8;
            val >>= 7;
            b |= (if val > 0 { 1 } else { 0 }) << 7;
            bytes.push(b);
            if val == 0 {
                break;
            }
        }
        serializer.serialize_bytes(&bytes[..])
    }
}

impl<'de> Deserialize<'de> for Varuint32 {
    fn deserialize<D>(deserializer: D) -> Result<Varuint32, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::mem::size_of;

        let mut n = 0;
        let mut shift = 0;
        let mut byte: u8 = Deserialize::deserialize(&mut deserializer)?;

        // Only allow reading size_of + 1 bytes to avoid overflows on bitshifts
        for _ in 0..(size_of::<usize>()) {
            if byte < 128 {
                break;
            }

            n |= ((byte & 127) as usize) << shift;
            shift += 7;
            byte = Deserialize::deserialize(deserializer)?;
        }

        let value = n | ((byte as usize) << shift);
        // Ok(Varuint32(value))
        // let s = String::deserialize(deserializer)?;
        Ok(Varuint32(value as u32))
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
