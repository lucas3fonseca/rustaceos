mod actions;
mod blocks;
mod requests;
mod results;

use anyhow::{anyhow, Error};
use eosio_cdt::eos::{eos_deserialize, eos_serialize};

pub use actions::*;
pub use blocks::*;
pub use requests::*;
pub use results::*;

pub trait Handler {
    fn request_bin(&mut self, bin: Vec<u8>);
    fn notify_head_lib(&self, head_lib: (u32, u32));
}

pub struct Ship<T: Handler> {
    block_num: u32,
    head_block: u32,
    lib: u32,
    abi_definitions: Option<String>,
    pub handler: T,
}

impl<T: Handler> Ship<T> {
    pub fn new(handler: T) -> Self {
        Ship {
            block_num: 0,
            head_block: 0,
            lib: 0,
            abi_definitions: None,
            handler,
        }
    }

    pub fn get_state(&self) -> (u32, u32) {
        (self.head_block, self.lib)
    }

    pub fn process_ship_text(&mut self, text: String) -> Result<(), Error> {
        if self.abi_definitions.is_none() {
            self.abi_definitions = Some(text);
            self.request_status()?;
            Ok(())
        } else {
            Err(anyhow!("unknown ship response\n{}", text))
        }
    }

    pub fn request_status(&mut self) -> Result<(), Error> {
        let status_request = ShipRequests::GetStatus(GetStatusRequestV0 {});
        self.handler.request_bin(eos_serialize(&status_request)?);
        Ok(())
    }

    fn update_status(&mut self, status: GetStatusResponseV0) {
        let initial_request = self.head_block == 0;
        self.head_block = status.block_position.block_num;
        self.lib = status.last_irreversible.block_num;
        self.handler.notify_head_lib((self.head_block, self.lib));

        if initial_request {
            self.request_block().expect("fail to request blocks");
        }
    }

    pub fn request_block(&mut self) -> Result<(), Error> {
        let block_request = ShipRequests::GetBlocks(GetBlocksRequestV0 {
            start_block_num: 1,
            end_block_num: 4294967295,
            max_messages_in_flight: 4294967295,
            have_positions: vec![],
            irreversible_only: false,
            fetch_block: true,
            fetch_traces: true,
            fetch_deltas: true,
        });
        self.handler.request_bin(eos_serialize(&block_request)?);
        Ok(())
    }

    fn update_block(&mut self, res: GetBlocksResultV0) {
        let head = res.head.block_num;
        let lib = res.last_irreversible.block_num;
        if self.head_block != head || self.lib != lib {
            self.head_block = head;
            self.lib = lib;
            self.handler.notify_head_lib((self.head_block, self.lib));
        }

        if let Some(block) = res.this_block {
            self.block_num = block.block_num;
            // TODO: notify block_num
            // TODO: parse and process block
        }
    }

    pub fn process_ship_binary(&mut self, bin: Vec<u8>) -> Result<(), Error> {
        let status_response: ShipResults =
            eos_deserialize(&bin).expect("fail to parse state history response");
        match status_response {
            ShipResults::GetStatus(res) => {
                self.update_status(res);
            }
            ShipResults::GetBlocks(res) => self.update_block(res),
        }
        Ok(())
    }
}
