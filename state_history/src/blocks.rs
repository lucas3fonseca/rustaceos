use bytes::{Buf, BufMut, Bytes, BytesMut};
use eosio_cdt::eos;
use eosio_cdt::eos::{Checksum256, Deserialize, PublicKey, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockPosition {
    pub block_num: u32,
    pub block_id: Checksum256,
}

// impl EosSerialize for BlockPosition {
//     fn read(buf: &mut Bytes) -> BlockPosition {
//         let block_num = u32::read(buf);
//         let block_id = Checksum256::read(buf);
//         BlockPosition {
//             block_num,
//             block_id,
//         }
//     }

//     // TODO
//     fn write(&self, bin: &mut BytesMut) {}
// }

#[derive(Debug, Serialize, Deserialize)]
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

// impl EosSerialize for BlockHeader {
//     fn read(buf: &mut Bytes) -> BlockHeader {
//         let timestamp = buf.get_u32_le();
//         let producer = buf.get_u64_le();
//         let confirmed = buf.get_u16_le();
//         let previous = Checksum256::read(buf);
//         let transaction_mroot = Checksum256::read(buf);
//         let action_mroot = Checksum256::read(buf);
//         let schedule_version = buf.get_u32_le();

//         let has_new_producers = buf.get_u8();
//         let new_producers = if has_new_producers == 1 {
//             Some(ProducerSchedule::read(buf))
//         } else {
//             None
//         };

//         let extensions_length = eos::read_varuint32(buf).unwrap();
//         if extensions_length > 0 {
//             panic!("todo: parse extensions");
//         }

//         BlockHeader {
//             timestamp,
//             producer,
//             confirmed,
//             previous,
//             transaction_mroot,
//             action_mroot,
//             schedule_version,
//             new_producers,
//             header_extensions: vec![],
//         }
//     }

//     // TODO
//     fn write(&self, buf: &mut BytesMut) {}
// }

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

// impl EosSerialize for ProducerSchedule {
//     fn read(buf: &mut Bytes) -> ProducerSchedule {
//         let version = buf.get_u32_le();

//         let producers_len = eos::read_varuint32(buf).unwrap();
//         let mut producers = Vec::new();
//         for _ in 0..producers_len {
//             producers.push(ProducerKey::read(buf));
//         }

//         ProducerSchedule { version, producers }
//     }

//     // TODO
//     fn write(&self, buf: &mut BytesMut) {}
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct ProducerKey {
    pub producer_name: u64, // todo: name
    pub block_signing_key: PublicKey,
}

// impl EosSerialize for ProducerKey {
//     fn read(buf: &mut Bytes) -> ProducerKey {
//         let producer_name = buf.get_u64_le();
//         let block_signing_key = PublicKey::read(buf);

//         ProducerKey {
//             producer_name,
//             block_signing_key,
//         }
//     }

//     // TODO
//     fn write(&self, buf: &mut BytesMut) {}
// }
