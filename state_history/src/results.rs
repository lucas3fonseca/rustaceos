use crate::actions::TransactionTraceV0;
use crate::blocks::{BlockHeader, BlockPosition};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use eosio_cdt::eos;
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

// impl EosSerialize for GetStatusResponseV0 {
//     fn read(buf: &mut Bytes) -> GetStatusResponseV0 {
//         let variant_index =
//             eos::read_varuint32(buf).expect("fail to read the get_status_response_v0 variant");
//         if variant_index != 0 {
//             panic!("the response does not refer to get_status_response_v0 variant");
//         }

//         let block_position = BlockPosition::read(buf);
//         let last_irreversible = BlockPosition::read(buf);
//         let trace_begin_block = buf.get_u32_le();
//         let trace_end_block = buf.get_u32_le();
//         let chain_state_begin_block = buf.get_u32_le();
//         let chain_state_end_block = buf.get_u32_le();

//         GetStatusResponseV0 {
//             block_position,
//             last_irreversible,
//             trace_begin_block,
//             trace_end_block,
//             chain_state_begin_block,
//             chain_state_end_block,
//         }
//     }

//     // TODO
//     fn write(&self, buf: &mut BytesMut) {}
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlocksResultV0 {
    pub head: BlockPosition,
    pub last_irreversible: BlockPosition,
    pub this_block: Option<BlockPosition>,
    pub prev_block: Option<BlockPosition>,
    pub block: Option<BlockHeader>,
    pub traces: Option<TransactionTraceV0>,
    pub deltas: Option<Vec<u8>>,
}

// impl EosSerialize for GetBlocksResultV0 {
//     fn read(buf: &mut Bytes) -> GetBlocksResultV0 {
//         let variant_index =
//             eos::read_varuint32(buf).expect("fail to read the get_blocks_result_v0 variant");
//         if variant_index != 1 {
//             panic!("the response does not refer to get_blocks_result_v0 variant");
//         }

//         let head = BlockPosition::read(buf);
//         let last_irreversible = BlockPosition::read(buf);

//         let this_block_present = buf.get_u8() == 1;
//         let this_block = if this_block_present {
//             let block = BlockPosition::read(buf);
//             Some(block)
//         } else {
//             None
//         };

//         let prev_block_present = buf.get_u8() == 1;
//         let prev_block = if prev_block_present {
//             let block = BlockPosition::read(buf);
//             Some(block)
//         } else {
//             None
//         };

//         let has_block = buf.get_u8() == 1;
//         let block = if has_block {
//             let block_header_length =
//                 eos::read_varuint32(buf).expect("fail to get block header bytes");
//             let mut block_header_bytes = Vec::with_capacity(block_header_length as usize);
//             buf.copy_to_slice(&mut block_header_bytes[..]);
//             let block_header = BlockHeader::read(buf);
//             Some(block_header)
//         } else {
//             None
//         };

//         let has_traces = buf.get_u8() == 1;
//         let traces = if has_traces {
//             let trace_length = eos::read_varuint32(buf).expect("fail to get trace length");
//             let mut trace_bytes = Vec::with_capacity(trace_length as usize);
//             buf.copy_to_slice(&mut trace_bytes[..]);
//             let traces = TransactionTraceV0::read(buf);
//             Some(traces)
//         } else {
//             None
//         };

//         GetBlocksResultV0 {
//             head,
//             last_irreversible,
//             this_block,
//             prev_block,
//             block,
//             traces,
//             deltas: None,
//         }
//     }

//     // TODO
//     fn write(&self, buf: &mut BytesMut) {}
// }
