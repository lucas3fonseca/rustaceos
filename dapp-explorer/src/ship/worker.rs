use log::{error, info};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
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

pub enum Msg {
    SetConnected,
    SetDisconnected,
    SetHeadLib(u32, u32),
    SetBlock(u32),
}

pub struct ShipWorker {
    link: AgentLink<ShipWorker>,
    subscribers: Vec<HandlerId>,
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
        info!("todo: implement websocket connection");
        let ws = WebSocket::new(ADDRESS).expect("fail to instantiate websocket");

        // create callback
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            // handle message
            let response = e
                .data()
                .as_string()
                .expect("Can't convert received data to a string");
            info!("message event, received data: {:?}", response);
        }) as Box<dyn FnMut(MessageEvent)>);
        // set message event handler on WebSocket
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        // forget the callback to keep it alive
        onmessage_callback.forget();

        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            error!("error event: {:?}", e);
            // self.update(Msg::SetDisconnected);
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let cloned_ws = ws.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            info!("socket opened");
            // self.update(Msg::SetConnected);

            match cloned_ws.send_with_str("ping") {
                Ok(_) => info!("message successfully sent"),
                Err(err) => error!("error sending message: {:?}", err),
            }
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }
}
