use crate::blocks::BlockPosition;
use abieos::{AbiDeserializer, Checksum256};
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
    fn deserialize(bin: &Vec<u8>) -> GetStatusResponseV0 {
        let mut buf = BytesMut::from(&bin[..]);

        let variant_index = abieos::read_varuint32(&mut buf)
            .expect("fail to read the get_status_response_v0 variant");
        if variant_index != 0 {
            panic!("the response does not refer to get_status_response_v0 variant");
        }

        let block_num = buf.get_u32_le();
        let mut block_id_bytes = [0; 32];
        buf.copy_to_slice(&mut block_id_bytes);
        let block_id = Checksum256 {
            value: block_id_bytes,
        };
        let block_position = BlockPosition {
            block_num,
            block_id,
        };

        let block_num = buf.get_u32_le();
        let mut block_id_bytes = [0; 32];
        buf.copy_to_slice(&mut block_id_bytes);
        let block_id = Checksum256 {
            value: block_id_bytes,
        };
        let last_irreversible = BlockPosition {
            block_num,
            block_id,
        };

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
    pub block: Option<Vec<u8>>,
    pub traces: Option<Vec<u8>>,
    pub deltas: Option<Vec<u8>>,
}

impl AbiDeserializer for GetBlocksResultV0 {
    fn deserialize(bin: &Vec<u8>) -> GetBlocksResultV0 {
        let mut buf = BytesMut::from(&bin[..]);

        let variant_index = abieos::read_varuint32(&mut buf)
            .expect("fail to read the get_blocks_result_v0 variant");
        if variant_index != 1 {
            panic!("the response does not refer to get_blocks_result_v0 variant");
        }

        let block_num = buf.get_u32_le();
        let mut block_id_bytes = [0; 32];
        buf.copy_to_slice(&mut block_id_bytes);
        let block_id = Checksum256 {
            value: block_id_bytes,
        };
        let head = BlockPosition {
            block_num,
            block_id,
        };

        let block_num = buf.get_u32_le();
        let mut block_id_bytes = [0; 32];
        buf.copy_to_slice(&mut block_id_bytes);
        let block_id = Checksum256 {
            value: block_id_bytes,
        };
        let last_irreversible = BlockPosition {
            block_num,
            block_id,
        };

        let this_block_present = buf.get_u8() == 1;
        let this_block = if this_block_present {
            let block_num = buf.get_u32_le();
            let mut block_id_bytes = [0; 32];
            buf.copy_to_slice(&mut block_id_bytes);
            let block_id = Checksum256 {
                value: block_id_bytes,
            };
            let block = BlockPosition {
                block_num,
                block_id,
            };
            Some(block)
        } else {
            None
        };

        let prev_block_present = buf.get_u8() == 1;
        let prev_block = if prev_block_present {
            let block_num = buf.get_u32_le();
            let mut block_id_bytes = [0; 32];
            buf.copy_to_slice(&mut block_id_bytes);
            let block_id = Checksum256 {
                value: block_id_bytes,
            };
            let block = BlockPosition {
                block_num,
                block_id,
            };
            Some(block)
        } else {
            None
        };

        GetBlocksResultV0 {
            head,
            last_irreversible,
            this_block,
            prev_block,
            block: None,
            traces: None,
            deltas: None,
        }
    }
}
