use bytes::{BytesMut, Buf};
use abieos::{AbiDeserializer, Checksum256, PublicKey, KeyType};

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

#[derive(Debug)]
pub struct TransactionTraceV0 {
  pub id: Checksum256,
  pub status: u8,
  pub cpu_usage_us: u32,
  pub net_usage_words: u32, // todo varuint32
  pub elapsed: i64,
  pub net_usage: u64,
  pub scheduled: bool,
  pub action_traces: Vec<ActionTraceV0>,
  pub account_ram_delta: Option<AccountDelta>,
  pub except: Option<String>,
  pub error_code: Option<u64>,
  pub failed_dtrx_trace: Option<Box<TransactionTraceV0>>,
  pub partial: Option<PartialTransactionV0>,
}

#[derive(Debug)]
pub struct PartialTransactionV0 {
  pub expiration: TimePointSec,
  pub ref_block_num: u16,
  pub ref_block_prefix: u32,
  pub max_net_usage_words: u32, // todo varuint32
  pub max_cpu_usage_ms: u8,
  pub delay_sec: u32, // todo varuint32
  pub transaction_extensions: Vec<Extension>,
  pub signatures: Vec<Signature>,
  pub context_free_data: Vec<u8>,
}

#[derive(Debug)]
pub struct TimePointSec {
  utc_seconds: u32,
}

#[derive(Debug)]
pub struct Signature {
  k1r1_size: u32, // always 65
  key_type: KeyType, // always k1
}

#[derive(Debug)]
pub struct ActionTraceV0 {
  pub action_ordinal: u32, // todo varuint32
  pub creator_action_ordinal: u32,
  pub receipt: Option<ActionReceipt>,
  pub receiver: u64, // name
  pub act: Action,
  pub context_free: bool,
  pub elapsed: i64,
  pub console: String,
  pub account_ram_deltas: Vec<AccountDelta>,
  pub except: Option<String>,
  pub error_code: Option<u64>,
}

#[derive(Debug)]
pub struct ActionReceipt {
  pub receiver: u64, // name
  pub act_digest: Checksum256,
  pub global_sequence: u64,
  pub recv_sequence: u64,
  pub auth_sequence: Vec<AccountAuthSequence>,
  pub code_sequence: u32, // todo varuint32
  pub abi_sequence: u32, // todo varuint32
}

#[derive(Debug)]
pub struct AccountAuthSequence {
  pub account: u64, // name
  pub sequence: u64,
}

#[derive(Debug)]
pub struct Action {
  pub account: u64, // name
  pub name: u64, // name
  pub authorization: Vec<PermissionLevel>,
  pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct PermissionLevel {
  pub actor: u64, // name
  pub permission: u64, // name
}

#[derive(Debug)]
pub struct AccountDelta {
  pub account: u64, // name
  pub delta: i64,
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
