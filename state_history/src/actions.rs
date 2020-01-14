use bytes::{BytesMut, Buf};
use abieos::{
  AbiDeserializer,
  Checksum256,
  TimePointSec,
  Signature,
  PermissionLevel,
};
use crate::blocks::Extension;

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

impl AbiDeserializer for TransactionTraceV0 {
  fn deserialize(buf: &mut BytesMut) -> TransactionTraceV0 {
    let id = abieos::read_checksum256(buf);
    let status = buf.get_u8();
    let cpu_usage_us = buf.get_u32_le();
    let net_usage_words = abieos::read_varuint32(buf).unwrap();
    let elapsed = buf.get_i64_le();
    let net_usage = buf.get_u64_le();
    let scheduled = buf.get_u8() != 0;
    let action_traces = vec![];

    TransactionTraceV0 {
      id,
      status,
      cpu_usage_us,
      net_usage_words,
      elapsed,
      net_usage,
      scheduled,
      action_traces,
      account_ram_delta: None,
      except: None,
      error_code: None,
      failed_dtrx_trace: None,
      partial: None,
    }
  }
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
pub struct AccountDelta {
  pub account: u64, // name
  pub delta: i64,
}
