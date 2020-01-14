use bytes::{BytesMut, Buf};
use abieos::{
  AbiDeserializer,
  Checksum256,
  PublicKey,
};

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
    pub timestamp: u32,
    pub producer: u64, // todo: name
    pub confirmed: u16,
    pub previous: Checksum256,
    pub transaction_mroot: Checksum256,
    pub action_mroot: Checksum256,
    pub schedule_version: u32,
    pub new_producers: Option<ProducerSchedule>,
    pub header_extensions: Vec<Extension>,
}

impl AbiDeserializer for BlockHeader {
    fn deserialize(buf: &mut BytesMut) -> BlockHeader {
        let timestamp = buf.get_u32_le();
        let producer = buf.get_u64_le();
        let confirmed = buf.get_u16_le();
        let previous = abieos::read_checksum256(buf);
        let transaction_mroot = abieos::read_checksum256(buf);
        let action_mroot = abieos::read_checksum256(buf);
        let schedule_version = buf.get_u32_le();

        let has_new_producers = buf.get_u8();
        let new_producers = if has_new_producers == 1 {
            Some(ProducerSchedule::deserialize(buf))
        } else {
            None
        };

        let extensions_length = abieos::read_varuint32(buf).unwrap();
        if extensions_length > 0 {
            panic!("todo: parse extensions");
        }

        BlockHeader {
            timestamp,
            producer,
            confirmed,
            previous,
            transaction_mroot,
            action_mroot,
            schedule_version,
            new_producers,
            header_extensions: vec![],
        }
    }
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

impl AbiDeserializer for ProducerSchedule {
    fn deserialize(buf: &mut BytesMut) -> ProducerSchedule {
        let version = buf.get_u32_le();

        let producers_len = abieos::read_varuint32(buf).unwrap();
        let mut producers = Vec::new();
        for _ in 0..producers_len {
            producers.push(ProducerKey::deserialize(buf));
        }

        ProducerSchedule {
            version,
            producers,
        }
    }
}

#[derive(Debug)]
pub struct ProducerKey {
  pub producer_name: u64, // todo: name
  pub block_signing_key: PublicKey,
}

impl AbiDeserializer for ProducerKey {
    fn deserialize(buf: &mut BytesMut) -> ProducerKey {
        let producer_name = buf.get_u64_le();
        let block_signing_key = PublicKey::deserialize(buf);

        ProducerKey {
            producer_name,
            block_signing_key,
        }
    }
}
