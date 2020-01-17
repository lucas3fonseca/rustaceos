use super::abi::AbiRead;
use bytes::{Buf, Bytes};
use std::fmt;

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

impl AbiRead for Checksum256 {
    fn read(buf: &mut Bytes) -> Checksum256 {
        let mut value = [0; 32];
        buf.copy_to_slice(&mut value);
        Checksum256 { value }
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
