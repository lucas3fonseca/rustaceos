// use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
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

// impl EosSerialize for Checksum256 {
//     fn read(buf: &mut Bytes) -> Checksum256 {
//         let mut value = [0; 32];
//         buf.copy_to_slice(&mut value);
//         Checksum256 { value }
//     }

//     fn write(&self, buf: &mut BytesMut) {
//         for v in &self.value {
//             buf.put_u8(*v);
//         }
//     }
// }

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
