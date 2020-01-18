// use super::EosSerialize;
// use bytes::{Buf, BufMut, Bytes, BytesMut};
use generic_array::typenum::{U33, U65};
use generic_array::{arr, GenericArray};
use serde::{Deserialize, Serialize};
use std::fmt;

big_array! {
    BigArray;
    +33,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Signature {
    K1(EccSignature),
    R1(EccSignature),
    WA(WebAuthnSignature),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EccSignature {
    data: GenericArray<u8, U65>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebAuthnSignature {
    compact_signature: EccSignature,
    auth_data: Vec<u8>,
    client_json: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PublicKey {
    K1(EccPublicKey),
    R1(EccPublicKey),
    WA(WebAuthnPublicKey),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EccPublicKey {
    pub data: GenericArray<u8, U33>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebAuthnPublicKey {
    key: EccPublicKey,
    user_presence: u8,
    rpid: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Signature {
//     k1r1_size: u32,    // always 65
//     key_type: KeyType, // always k1
// }

// impl fmt::Debug for PublicKey {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "PubKey {:?} {:?}", self.r#type, &self.data[..32])
//     }
// }

// #[test]
// fn test_keys_deserializes_properly() {
//     let public_key = PublicKey {
//         r#type: KeyType::R1,
//         data: arr![u8; 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
//     };
//     let bytes = bincode::serialize(&public_key).unwrap();
//     assert_eq!(
//         bytes,
//         vec![
//             1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
//             5, 5, 5, 5, 5
//         ]
//     );
// }

// impl EosSerialize for PublicKey {
//     fn read(buf: &mut Bytes) -> PublicKey {
//         let key_type_index = buf.get_u8();
//         let key_type = match key_type_index {
//             0 => KeyType::K1,
//             1 => KeyType::R1,
//             2 => panic!("TODO: implement web authn key"),
//             _ => panic!("Invalid key type for public key"),
//         };
//         let mut data = [0; 33];

//         for n in 0..data.len() {
//             data[n] = buf.get_u8();
//         }

//         PublicKey {
//             r#type: key_type,
//             data,
//         }
//     }

//     // TODO
//     fn write(&self, buf: &mut BytesMut) {
//         buf.put_u8(0);
//     }
// }
