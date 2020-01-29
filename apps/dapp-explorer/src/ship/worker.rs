use anyhow::Error;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::result::Result;
use yew::format::Binary;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::Task;
use yew::worker::*;
use yew::Callback;

use state_history::{Handler as ShipHandler, Ship};

static ADDRESS: &str = "ws://localhost:8080";

#[derive(Eq, PartialEq)]
pub enum ConnectionStatus {
    Offline,
    Online,
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
    NotifyHeadLib,
    WsResponse(ResponseType),
}

struct ShipTask {
    ws: WebSocketTask,
    head_lib_callback: Callback<(u32, u32)>,
}

impl ShipHandler for ShipTask {
    fn request_bin(&mut self, bin: Vec<u8>) {
        let bin: Binary = Ok(bin);
        self.ws.send_binary(bin);
    }

    fn notify_head_lib(&self, head_lib: (u32, u32)) {
        self.head_lib_callback.emit(head_lib);
    }
}

pub struct ShipWorker {
    link: AgentLink<ShipWorker>,
    subscribers: Vec<HandlerId>,
    ws_service: WebSocketService,
    status: ConnectionStatus,
    ship: Option<Ship<ShipTask>>,
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
            // ws: None,
            subscribers: vec![],
            status: ConnectionStatus::Offline,
            ship: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::SetConnected => {
                self.status = ConnectionStatus::Online;
                self.notify_subscribers(Response::Connected);
            }
            Msg::SetDisconnected => {
                self.status = ConnectionStatus::Offline;
                self.ship = None;
                // self.ws = None;
                self.notify_subscribers(Response::Disconnected);
            }
            Msg::NotifyHeadLib => {
                if let Some(ship) = &self.ship {
                    let (head, lib) = ship.get_state();
                    self.notify_subscribers(Response::UpdatedHeadLib(head, lib));
                }
            }
            Msg::WsResponse(data) => {
                if let Some(ship) = &mut self.ship {
                    match data {
                        ResponseType::Binary(bin) => ship
                            .process_ship_binary(bin)
                            .unwrap_or_else(|_| self.disconnect()),
                        ResponseType::Text(txt) => ship
                            .process_ship_text(txt)
                            .unwrap_or_else(|_| self.disconnect()),
                        ResponseType::Error(e) => {
                            error!("ship ws got error, disconnecting...\n{:?}", e);
                            self.disconnect();
                        }
                    }
                }
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
        if self.ship.is_some() {
            warn!("ship websocket connection is in progress...");
            return;
        }

        let ws_callback = self
            .link
            .callback(|data: ResponseType| Msg::WsResponse(data));

        let notification_callback = self.link.callback(|status| match status {
            WebSocketStatus::Opened => Msg::SetConnected,
            WebSocketStatus::Closed | WebSocketStatus::Error => Msg::SetDisconnected,
        });

        let ws = self
            .ws_service
            .connect(ADDRESS, ws_callback, notification_callback)
            .expect("fail to instantiate websocket");

        let head_lib_callback = self
            .link
            .callback(|_head_lib: (u32, u32)| Msg::NotifyHeadLib);

        let ship_task = ShipTask {
            ws,
            head_lib_callback,
        };

        self.ship = Some(Ship::new(ship_task));
    }

    fn disconnect(&mut self) {
        if self.ship.is_none() {
            warn!("ship websocket is already off...");
        } else {
            self.ship.take().unwrap().handler.ws.cancel();
            self.ship = None;
        }
    }
}
