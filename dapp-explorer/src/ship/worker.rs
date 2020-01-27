use anyhow::Error;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::result::Result;
use yew::format::Binary;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::Task;
use yew::worker::*;

use eosio_cdt::eos::{eos_deserialize, eos_serialize};
use state_history::{
    GetBlocksRequestV0, GetStatusRequestV0, ShipRequests, ShipResults, SignedBlockHeader, Traces,
};

static ADDRESS: &str = "ws://localhost:8080";
static INIT_BLOCK: u32 = 1;
static END_BLOCK: u32 = 4294967294;

#[derive(Eq, PartialEq)]
pub enum ConnectionStatus {
    Offline,
    Online,
}

pub struct Ship {
    block_num: u32,
    head_block: u32,
    lib: u32,
    abi_definitions: Option<String>,
    status: ConnectionStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Connect,
    Subscribe,
    Unsubscribe,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Response {
    Connected,
    Disconnected,
    UpdatedHeadLib(u32, u32),
}

#[derive(Debug)]
pub enum ResponseType {
    Text(String),
    Binary(Vec<u8>),
    Error(Error),
}

impl From<Result<String, Error>> for ResponseType {
    fn from(data: Result<String, Error>) -> Self {
        match data {
            Ok(data) => ResponseType::Text(data),
            Err(e) => ResponseType::Error(e),
        }
    }
}

impl From<Result<Vec<u8>, Error>> for ResponseType {
    fn from(data: Result<Vec<u8>, Error>) -> Self {
        match data {
            Ok(data) => ResponseType::Binary(data),
            Err(e) => ResponseType::Error(e),
        }
    }
}

pub enum Msg {
    SetConnected,
    SetDisconnected,
    SetHeadLib(u32, u32),
    SetBlock(u32),
    WsResponse(ResponseType), //Result<ResponseType, Error>),
}

pub struct ShipWorker {
    link: AgentLink<ShipWorker>,
    subscribers: Vec<HandlerId>,
    ws_service: WebSocketService,
    ws: Option<WebSocketTask>,
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
            ws_service: WebSocketService::new(),
            ws: None,
            subscribers: vec![],
            data: Ship {
                abi_definitions: None,
                block_num: 0,
                head_block: 0,
                lib: 0,
                status: ConnectionStatus::Offline,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::SetConnected => {
                self.data.status = ConnectionStatus::Online;
                self.notify_subscribers(Response::Connected);
            }
            Msg::SetDisconnected => {
                self.data.status = ConnectionStatus::Offline;
                self.ws = None;
                self.notify_subscribers(Response::Disconnected);
            }
            Msg::SetHeadLib(head_block, lib) => {
                self.data.head_block = head_block;
                self.data.lib = lib;
                self.notify_subscribers(Response::UpdatedHeadLib(head_block, lib));
            }
            Msg::SetBlock(block_num) => self.data.block_num = block_num,
            Msg::WsResponse(data) => match data {
                ResponseType::Binary(bin) => self.process_ship_binary(bin),
                ResponseType::Text(txt) => self.process_ship_text(txt),
                ResponseType::Error(e) => {
                    error!("ship ws got error, disconnecting...\n{:?}", e);
                    self.disconnect()
                }
            },
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::Connect => self.connect(),
            Request::Subscribe => self.subscribers.push(who),
            Request::Unsubscribe => self.subscribers.retain(|&i| i != who),
        }
    }
}

impl ShipWorker {
    fn notify_subscribers(&self, response: Response) {
        self.subscribers
            .iter()
            .for_each(|&i| self.link.respond(i, response));
    }

    fn process_ship_text(&mut self, text: String) {
        if self.data.abi_definitions.is_none() {
            self.data.abi_definitions = Some(text);
            info!("initialized abi definitions, requesting status...");
            let status_request = ShipRequests::GetStatus(GetStatusRequestV0 {});
            let bin: Binary = eos_serialize(&status_request).map_err(Error::msg);
            self.ws.as_mut().unwrap().send_binary(bin);
        } else {
            info!("unknown ship response\n{}", text);
        }
    }

    fn process_ship_binary(&mut self, bin: Vec<u8>) {
        let status_response: ShipResults =
            eos_deserialize(&bin).expect("fail to parse state history response");
        match status_response {
            ShipResults::GetStatus(res) => {
                info!("received initial ship status, requesting blocks...");
                self.update(Msg::SetHeadLib(
                    res.block_position.block_num,
                    res.last_irreversible.block_num,
                ));

                let block_request = ShipRequests::GetBlocks(GetBlocksRequestV0 {
                    start_block_num: INIT_BLOCK,
                    end_block_num: END_BLOCK + 1,
                    max_messages_in_flight: 4294967295,
                    have_positions: vec![],
                    irreversible_only: false,
                    fetch_block: true,
                    fetch_traces: true,
                    fetch_deltas: true,
                });
                let bin: Binary = eos_serialize(&block_request).map_err(Error::msg);
                self.ws.as_mut().unwrap().send_binary(bin);
            }
            ShipResults::GetBlocks(res) => {
                let head = res.head.block_num;
                let lib = res.last_irreversible.block_num;
                if self.data.head_block != head {
                    self.update(Msg::SetHeadLib(head, lib));
                }

                if let Some(block) = res.this_block {
                    self.update(Msg::SetBlock(block.block_num));
                }
            }
        }
    }

    fn connect(&mut self) {
        if self.ws.is_some() {
            warn!("ship websocket connection is in progress...");
            return;
        }

        let callback = self
            .link
            .callback(|data: ResponseType| Msg::WsResponse(data));
        let notification = self.link.callback(|status| match status {
            WebSocketStatus::Opened => Msg::SetConnected,
            WebSocketStatus::Closed | WebSocketStatus::Error => Msg::SetDisconnected,
        });

        let task = self
            .ws_service
            .connect(ADDRESS, callback, notification)
            .expect("fail to instantiate websocket");
        self.ws = Some(task);
    }

    fn disconnect(&mut self) {
        if self.ws.is_none() {
            warn!("ship websocket is already off...");
        } else {
            self.ws.take().unwrap().cancel();
        }
    }
}
