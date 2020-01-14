use std::fmt;
use bytes::{BytesMut, Buf};

pub trait AbiSerializer {
    fn serialize(&self) -> Vec<u8>;
}

pub trait AbiDeserializer {
    fn deserialize(buf: &mut BytesMut) -> Self;
}

pub struct Variant<'a> {
    pub name: &'a str,
    pub types: Vec<&'a str>,
}

#[derive(Debug)]
pub enum KeyType {
    K1 = 0,
    R1 = 1,
}

pub struct PublicKey {
    pub r#type: KeyType,
    pub data: [u8; 33],
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PubKey {:?} {:?}", self.r#type, &self.data[..32])
    }
}

pub struct Checksum256 {
    pub value: [u8; 32],
}

impl Checksum256 {
    pub fn to_string(&self) -> String {
        let mut hex_string = String::from("");
        for v in &self.value {
            let hex = format!("{:02x}", v);
            hex_string += hex.as_str();
        }
        hex_string
    }
}

impl fmt::Display for Checksum256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Checksum256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

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

pub fn read_varuint32(bin: &mut BytesMut) -> Result<u32, &'static str> {
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
        if shift >= 35 { return Err("invalid varuint32 encoding"); }
    }

    Ok(value)
}

pub fn read_checksum256(buf: &mut BytesMut) -> Checksum256 {
    let mut value = [0; 32];
    buf.copy_to_slice(&mut value);
    Checksum256 { value }
}
