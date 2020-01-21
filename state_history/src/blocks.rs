use eosio_cdt::eos::{Checksum256, Deserialize, Name, PublicKey, Serialize, Signature};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockPosition {
    pub block_num: u32,
    pub block_id: Checksum256,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    pub timestamp: u32,
    pub producer: Name,
    pub confirmed: u16,
    pub previous: Checksum256,
    pub transaction_mroot: Checksum256,
    pub action_mroot: Checksum256,
    pub schedule_version: u32,
    pub new_producers: Option<ProducerSchedule>,
    pub header_extensions: Vec<Extension>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedBlockHeader {
    pub header: BlockHeader,
    pub producer_signature: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extension {
    pub r#type: u16,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProducerSchedule {
    pub version: u32,
    pub producers: Vec<ProducerKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProducerKey {
    pub producer_name: Name,
    pub block_signing_key: PublicKey,
}
