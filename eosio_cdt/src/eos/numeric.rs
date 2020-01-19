use std::fmt;

use bytes::{Buf, Bytes};
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct Varuint32(u32);

impl Serialize for Varuint32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes: Vec<u8> = Vec::new();
        let mut val: u64 = self.0 as u64;
        loop {
            let mut b: u8 = (val as u8) & 0x7f;
            val >>= 7;
            b |= ((val > 0) as u8) << 7;
            bytes.push(b);
            if val == 0 {
                break;
            }
        }

        let mut tup = serializer.serialize_tuple(bytes.len())?;
        for b in &bytes {
            tup.serialize_element(b)?;
        }
        tup.end()
    }
}

impl<'de> Deserialize<'de> for Varuint32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {

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

                while let Some(elem) = seq.next_element()? {
                    let byte: u8 = elem;

                    if byte < 128 {
                        break;
                    }

                    n |= ((byte & 127) as usize) << shift;
                    shift += 7;

                    final_byte = byte;
                }

                // while let Some(byte) = seq.next_element::<u8>()? {
                // let mut byte_opt = seq.next_element::<u8>()?;
                // while byte_opt.is_some() {
                //     let byte = byte_opt.unwrap();
                //     if byte < 128 {
                //         break;
                //     }

                //     n |= ((byte & 127) as usize) << shift;
                //     shift += 7;

                //     final_byte = byte;

                //     byte_opt = seq.next_element::<u8>()?;
                // }

                let value = n | ((final_byte as usize) << shift);

                println!("varuint val {}", value);
                Ok(Varuint32(value as u32))
            }
        }

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

#[test]
fn test_varuint32_serialization() {
    use super::eos_serialize;

    let zero = Varuint32(0);
    assert_eq!(eos_serialize(&zero).unwrap(), [0]);

    let one = Varuint32(1);
    assert_eq!(eos_serialize(&one).unwrap(), [1]);

    let v230 = Varuint32(230);
    assert_eq!(eos_serialize(&v230).unwrap(), [230, 1]);

    let v2048 = Varuint32(2048);
    assert_eq!(eos_serialize(&v2048).unwrap(), [128, 16]);

    let full = Varuint32(4294967295);
    assert_eq!(eos_serialize(&full).unwrap(), [255, 255, 255, 255, 15]);
}
