use std::fmt;

use bytes::{Buf, Bytes};
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
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

struct Varuint32Visitor;

impl<'de> Visitor<'de> for Varuint32Visitor {
    type Value = Varuint32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a variant integer")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut n = 0;
        let mut shift = 0;

        let mut final_byte = 0;
        while let Some(byte) = seq.next_element::<u8>()? {
            if byte < 128 {
                break;
            }

            n |= ((byte & 127) as usize) << shift;
            shift += 7;

            final_byte = byte;
        }

        let value = n | ((final_byte as usize) << shift);

        println!("varuint val {}", value);
        Ok(Varuint32(value as u32))
    }
}

impl<'de> Deserialize<'de> for Varuint32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(Varuint32Visitor)
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
