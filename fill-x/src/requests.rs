use abieos::Variant;

pub static ShipRequests = Variant {
  name = "request",
  types = vec![
    "get_status_request_v0",
    "get_blocks_request_v0",
    "get_blocks_ack_request_v0"
  ]
}

pub struct BlockPosition {
  block_num: u32,
  block_id: String,
}

pub struct GetStatusRequest;

pub struct GetBlocksRequest {
  pub start_block_num: u32,
  pub end_block_num: u32,
  pub max_messages_in_flight: u32,
  pub have_positions: Vec<BlockPosition>,
  pub irreversible_only: bool,
  pub fetch_block: bool,
  pub fetch_traces: bool,
  pub fetch_deltas: bool,
}
