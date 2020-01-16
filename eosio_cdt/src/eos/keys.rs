use bytes::{Bytes, Buf};
use std::fmt;

#[derive(Debug)]
pub enum KeyType {
    K1 = 0,
    R1 = 1,
    WA = 2,
}

pub struct PublicKey {
    pub r#type: KeyType,
    pub data: [u8; 33],
}

#[derive(Debug)]
pub struct Signature {
  k1r1_size: u32, // always 65
  key_type: KeyType, // always k1
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PubKey {:?} {:?}", self.r#type, &self.data[..32])
    }
}

impl AbiRead for PublicKey {
    fn read(buf: &mut Bytes) -> PublicKey {
        let key_type_index = buf.get_u8();
        let key_type = match key_type_index {
            0 => KeyType::K1,
            1 => KeyType::R1,
            2 => panic!("TODO: implement web authn key"),
            _ => panic!("Invalid key type for public key"),
        };
        let mut data = [0; 33];

        for n in 0..data.len() {
            data[n] = buf.get_u8();
        }

        PublicKey {
            r#type: key_type,
            data,
        }
    }
}
