use crate::blocks::BlockPosition;
use eosio_cdt::eos::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ShipRequests {
    GetStatus(GetStatusRequestV0),
    GetBlocks(GetBlocksRequestV0),
}

#[derive(Serialize, Deserialize)]
pub struct GetStatusRequestV0;

#[derive(Serialize, Deserialize)]
pub struct GetBlocksRequestV0 {
    pub start_block_num: u32,
    pub end_block_num: u32,
    pub max_messages_in_flight: u32,
    pub have_positions: Vec<BlockPosition>,
    pub irreversible_only: bool,
    pub fetch_block: bool,
    pub fetch_traces: bool,
    pub fetch_deltas: bool,
}
