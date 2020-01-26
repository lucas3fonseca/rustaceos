use anyhow::Error;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use yew::format::{Binary, Format, Json};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::worker::*;

use eosio_cdt::eos::{eos_deserialize, eos_serialize};
use state_history::{
    GetBlocksRequestV0, GetStatusRequestV0, ShipRequests, ShipResults, SignedBlockHeader, Traces,
};

static ADDRESS: &str = "ws://localhost:8080";
static INIT_BLOCK: u32 = 1;
static END_BLOCK: u32 = 30;

#[derive(Eq, PartialEq)]
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
    Subscribe,
    Unsubscribe,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Response {
    Connected,
    Disconnected,
}

#[derive(Deserialize, Debug)]
pub struct ResponseType {
    data: String,
}

pub enum Msg {
    SetConnected,
    SetDisconnected,
    SetHeadLib(u32, u32),
    SetBlock(u32),
    WsResponse(Result<ResponseType, Error>),
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
                self.notify_subscribers(Response::Disconnected);
            }
            Msg::SetHeadLib(head_block, lib) => {
                self.data.head_block = head_block;
                self.data.lib = lib;
            }
            Msg::SetBlock(block_num) => self.data.block_num = block_num,
            Msg::WsResponse(data) => {
                info!("got some ws response {:?}", data);
            }
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

    fn connect(&mut self) {
        if self.ws.is_some() {
            warn!("ship websocket connection is in progress...");
            return;
        }

        let callback = self.link.callback(|Json(data)| {
            // let response = match data {
            //     Ok(data) => Ok(ResponseType::Text(data)),
            //     // Ok(Binary(data)) => Ok(ResponseType::Binary(data)),
            //     Err(e) => Err(e),
            // };
            Msg::WsResponse(data)
        });
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
}
