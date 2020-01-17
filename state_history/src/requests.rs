use crate::blocks::BlockPosition;
use bytes::BufMut;
use eosio_cdt::eos;
use eosio_cdt::eos::AbiWrite;

// to use in the future when we generalize the requests functions
// pub static ShipRequests: Variant = Variant {
//   name: "request",
//   types: [
//     "get_status_request_v0",
//     "get_blocks_request_v0",
//     "get_blocks_ack_request_v0",
//   ],
// };

pub struct GetStatusRequestV0;

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

impl AbiWrite for GetStatusRequestV0 {
    fn write(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        // todo: find it in static requests variant
        let get_status_variant_index = 0;
        eos::push_varuint32(&mut buffer, get_status_variant_index);

        buffer
    }
}

impl AbiWrite for GetBlocksRequestV0 {
    fn write(&self) -> Vec<u8> {
        let mut buf = vec![];

        // todo: find it in static requests variant
        let get_status_variant_index = 1;
        buf.put_u8(get_status_variant_index);

        buf.put_u32_le(self.start_block_num);
        buf.put_u32_le(self.end_block_num);
        buf.put_u32_le(self.max_messages_in_flight);

        eos::push_varuint32(&mut buf, self.have_positions.len() as u32);
        for pos in &self.have_positions {
            buf.put_u32_le(pos.block_num);
            for v in &pos.block_id.value {
                buf.put_u8(*v);
            }
        }

        buf.put_u8(if self.irreversible_only { 1 } else { 0 });
        buf.put_u8(if self.fetch_block { 1 } else { 0 });
        buf.put_u8(if self.fetch_traces { 1 } else { 0 });
        buf.put_u8(if self.fetch_deltas { 1 } else { 0 });

        buf
    }
}
