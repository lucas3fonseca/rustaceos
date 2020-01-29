use crate::blocks::BlockPosition;
use eosio_cdt::eos::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ShipResults {
    GetStatus(GetStatusResponseV0),
    GetBlocks(GetBlocksResultV0),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStatusResponseV0 {
    pub block_position: BlockPosition,
    pub last_irreversible: BlockPosition,
    pub trace_begin_block: u32,
    pub trace_end_block: u32,
    pub chain_state_begin_block: u32,
    pub chain_state_end_block: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlocksResultV0 {
    pub head: BlockPosition,
    pub last_irreversible: BlockPosition,
    pub this_block: Option<BlockPosition>,
    pub prev_block: Option<BlockPosition>,
    pub block: Option<Vec<u8>>,
    pub traces: Option<Vec<u8>>,
    pub deltas: Option<Vec<u8>>,
}
