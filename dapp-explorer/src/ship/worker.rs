use log::info;
use serde::{Deserialize, Serialize};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use yew::worker::*;

use eosio_cdt::eos::{eos_deserialize, eos_serialize};
use state_history::{
    GetBlocksRequestV0, GetStatusRequestV0, ShipRequests, ShipResults, SignedBlockHeader, Traces,
};

static ADDRESS: &str = "http://localhost:8080";
static INIT_BLOCK: u32 = 1;
static END_BLOCK: u32 = 30;

pub enum ConnectionStatus {
    Offline,
    Online,
}

pub struct Ship {
    block_num: u32,
    head_block: u32,
    lib: u32,
    status: ConnectionStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Connect,
    Question(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Answer(String),
}

pub enum Msg {
    SetConnected,
    SetDisconnected,
    SetHeadLib(u32, u32),
    SetBlock(u32),
}

pub struct ShipWorker {
    link: AgentLink<ShipWorker>,
    data: Ship,
}

impl Agent for ShipWorker {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        ShipWorker {
            link,
            data: Ship {
                block_num: 0,
                head_block: 0,
                lib: 0,
                status: ConnectionStatus::Offline,
            },
        }
    }

    // Handle inner messages (from callbacks)
    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::SetConnected => self.data.status = ConnectionStatus::Online,
            Msg::SetDisconnected => self.data.status = ConnectionStatus::Offline,
            Msg::SetHeadLib(head_block, lib) => {
                self.data.head_block = head_block;
                self.data.lib = lib;
            }
            Msg::SetBlock(block_num) => self.data.block_num = block_num,
        }
    }

    // Handle incoming messages from components of other agents.
    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::Connect => self.connect(),
            Request::Question(_) => {
                self.link
                    .respond(who, Response::Answer("That's cool!".into()));
            }
        }
    }
}

impl ShipWorker {
    fn connect(&mut self) {
        info!("todo: implement websocket connection");
    }
}
