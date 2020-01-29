use crate::blocks::Extension;
use eosio_cdt::eos::{
    Checksum256, Deserialize, Name, PermissionLevel, Serialize, Signature, TimePointSec, Varuint32,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Traces {
    TransactionTrace(TransactionTraceV0),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionTraceV0 {
    pub id: Checksum256,
    pub status: u8,
    pub cpu_usage_us: u32,
    pub net_usage_words: Varuint32,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialTransactionV0 {
    pub expiration: TimePointSec,
    pub ref_block_num: u16,
    pub ref_block_prefix: u32,
    pub max_net_usage_words: Varuint32,
    pub max_cpu_usage_ms: u8,
    pub delay_sec: Varuint32,
    pub transaction_extensions: Vec<Extension>,
    pub signatures: Vec<Signature>,
    pub context_free_data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionTraceV0 {
    pub action_ordinal: Varuint32,
    pub creator_action_ordinal: u32,
    pub receipt: Option<ActionReceipt>,
    pub receiver: Name,
    pub act: Action,
    // pub context_free: bool,
    // pub elapsed: i64,
    // pub console: String,
    // pub account_ram_deltas: Vec<AccountDelta>,
    // pub except: Option<String>,
    // pub error_code: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionReceipt {
    pub receiver: Name,
    pub act_digest: Checksum256,
    pub global_sequence: u64,
    pub recv_sequence: u64,
    pub auth_sequence: Vec<AccountAuthSequence>,
    pub code_sequence: Varuint32,
    pub abi_sequence: Varuint32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAuthSequence {
    pub account: Name,
    pub sequence: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub account: Name,
    pub name: Name,
    pub authorization: Vec<PermissionLevel>,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDelta {
    pub account: Name,
    pub delta: i64,
}
