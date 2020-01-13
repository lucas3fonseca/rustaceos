use bytes::Buf;

// use abieos::Variant;

// pub static ShipRequests: Variant = Variant {
//   name: "request",
//   types: vec![
//     "get_status_request_v0",
//     "get_blocks_request_v0",
//     "get_blocks_ack_request_v0"
//   ],
// };

#[derive(Debug)]
pub struct BlockPosition {
  pub block_num: u32,
  pub block_id: String,
}

pub trait AbiSerializer {
  fn serialize(&self) -> Vec<u8>;
}

pub trait AbiDeserializer {
  fn deserialize(bin: Vec<u8>) -> Self;
}

pub struct GetStatusRequest;

impl AbiSerializer for GetStatusRequest {
  fn serialize(&self) -> Vec<u8> {
    let mut buffer = Vec::new();
    let get_status_variant_index = 0;
    abieos::push_varuint32(&mut buffer, get_status_variant_index);
    buffer
  }
}

#[derive(Debug)]
pub struct GetStatusResponse {
  pub block_position: BlockPosition,
  pub last_irreversible: BlockPosition,
  pub trace_begin_block: u32,
  pub trace_end_block: u32,
  pub chain_state_begin_block: u32,
  pub chain_state_end_block: u32,
}

impl AbiDeserializer for GetStatusResponse {
  fn deserialize(bin: Vec<u8>) -> GetStatusResponse {

    let mut buf = &bin[..];

    let variant_index = buf.get_u8();
    if variant_index != 0 { panic!("the response does not refer to get_status_response_v0 variant"); }

    let block_num = buf.get_u32_le();
    println!("blocknum is {}", block_num);

    let mut block_id_bytes = [0; 32];
    buf.copy_to_slice(&mut block_id_bytes);
    println!("block_id_bytes is {:?}", block_id_bytes);
    let block_position = BlockPosition { block_num, block_id: String::from("0") };

    let block_num = buf.get_u32_le();
    let mut block_id_bytes = [0; 32];
    buf.copy_to_slice(&mut block_id_bytes);
    let last_irreversible = BlockPosition { block_num, block_id: String::from("0") };

    let trace_begin_block = buf.get_u32_le();
    let trace_end_block = buf.get_u32_le();
    let chain_state_begin_block = buf.get_u32_le();
    let chain_state_end_block = buf.get_u32_le();

    GetStatusResponse {
      block_position,
      last_irreversible,
      trace_begin_block,
      trace_end_block,
      chain_state_begin_block,
      chain_state_end_block,
    }
  }
}

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
