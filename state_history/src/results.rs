use crate::blocks::{
  BlockPosition,
  BlockHeader,
  ActionTrace,
};
use abieos::{AbiDeserializer};
use bytes::{Buf, BytesMut};

#[derive(Debug)]
pub struct GetStatusResponseV0 {
    pub block_position: BlockPosition,
    pub last_irreversible: BlockPosition,
    pub trace_begin_block: u32,
    pub trace_end_block: u32,
    pub chain_state_begin_block: u32,
    pub chain_state_end_block: u32,
}

impl AbiDeserializer for GetStatusResponseV0 {
    fn deserialize(buf: &mut BytesMut) -> GetStatusResponseV0 {
        let variant_index = abieos::read_varuint32(buf)
            .expect("fail to read the get_status_response_v0 variant");
        if variant_index != 0 {
            panic!("the response does not refer to get_status_response_v0 variant");
        }

        let block_position = BlockPosition::deserialize(buf);
        let last_irreversible = BlockPosition::deserialize(buf);
        let trace_begin_block = buf.get_u32_le();
        let trace_end_block = buf.get_u32_le();
        let chain_state_begin_block = buf.get_u32_le();
        let chain_state_end_block = buf.get_u32_le();

        GetStatusResponseV0 {
            block_position,
            last_irreversible,
            trace_begin_block,
            trace_end_block,
            chain_state_begin_block,
            chain_state_end_block,
        }
    }
}

#[derive(Debug)]
pub struct GetBlocksResultV0 {
    pub head: BlockPosition,
    pub last_irreversible: BlockPosition,
    pub this_block: Option<BlockPosition>,
    pub prev_block: Option<BlockPosition>,
    pub block: Option<BlockHeader>,
    pub traces: Option<Vec<u8>>,
    pub deltas: Option<Vec<u8>>,
}

impl AbiDeserializer for GetBlocksResultV0 {
    fn deserialize(buf: &mut BytesMut) -> GetBlocksResultV0 {
        let variant_index = abieos::read_varuint32(buf)
            .expect("fail to read the get_blocks_result_v0 variant");
        if variant_index != 1 {
            panic!("the response does not refer to get_blocks_result_v0 variant");
        }

        let head = BlockPosition::deserialize(buf);
        let last_irreversible = BlockPosition::deserialize(buf);

        let this_block_present = buf.get_u8() == 1;
        let this_block = if this_block_present {
            let block = BlockPosition::deserialize(buf);
            Some(block)
        } else {
            None
        };

        let prev_block_present = buf.get_u8() == 1;
        let prev_block = if prev_block_present {
            let block = BlockPosition::deserialize(buf);
            Some(block)
        } else {
            None
        };

        let has_block = buf.get_u8() == 1;
        let block = if has_block {
            let block_header_length = abieos::read_varuint32(buf)
                .expect("fail to get block header bytes");
            let mut block_header_bytes = Vec::with_capacity(block_header_length as usize);
            buf.copy_to_slice(&mut block_header_bytes[..]);
            let block_header = BlockHeader::deserialize(buf);
            Some(block_header)
        } else {
            None
        };

        let has_traces = buf.get_u8() == 1;

        GetBlocksResultV0 {
            head,
            last_irreversible,
            this_block,
            prev_block,
            block,
            traces: None,
            deltas: None,
        }
    }
}
