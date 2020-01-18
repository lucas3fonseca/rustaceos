use super::{Deserialize, Serialize};
use generic_array::typenum::{U33, U65};
use generic_array::GenericArray;

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
