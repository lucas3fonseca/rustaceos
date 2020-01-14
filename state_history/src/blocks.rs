use bytes::{BytesMut, Buf};
use abieos::{AbiDeserializer, Checksum256, PublicKey};

#[derive(Debug)]
pub struct BlockPosition {
    pub block_num: u32,
    pub block_id: Checksum256,
}

impl AbiDeserializer for BlockPosition {
  fn deserialize(buf: &mut BytesMut) -> BlockPosition {
    let block_num = buf.get_u32_le();
    let block_id = abieos::read_checksum256(buf);
    BlockPosition {
        block_num,
        block_id,
    }
  }
}

#[derive(Debug)]
pub struct BlockHeader {
    pub timestamp: BlockPosition,
    pub producer: u64, // todo: name
    pub confirmed: u16,
    pub previous: Checksum256,
    pub transaction_mroot: Checksum256,
    pub action_mroot: Checksum256,
    pub schedule_version: u32,
    pub new_producers: Option<ProducerSchedule>,
    pub header_extensions: Vec<Extension>,
}

#[derive(Debug)]
pub struct Extension {
  pub r#type: u16,
  pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct ProducerSchedule {
  pub version: u32,
  pub producers: Vec<ProducerKey>,
}

#[derive(Debug)]
pub struct ProducerKey {
  pub ProducerName: u64, // todo: name
  pub BlockSigningKey: PublicKey,
}
