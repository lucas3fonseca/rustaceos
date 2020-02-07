use js_sys::Uint8Array;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{BinaryType, CloseEvent, Event, ErrorEvent, MessageEvent, WebSocket, Window};
use gloo::events::EventListener;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Howdy, {}!", name));
}

use state_history::{Handler, Ship};

#[derive(Debug)]
pub enum WebSocketStatus {
    Opened(Event),
    Closed(CloseEvent),
    Error(ErrorEvent),
}

const SHIP_URL: &'static str = "ws://localhost:8080";

struct Worker {
    window: Window,
}

impl Worker {
    fn new(window: Window) -> Self {
        Worker { window }
    }

    // fn start(&mut self) {
    //     let interval = Closure::wrap(Box::new(move || self.process_loop()) as Box<dyn Fn()>);
    //     self.window.set_interval_with_callback_and_timeout_and_arguments_0(interval.as_ref().unchecked_ref(), 1000).unwrap();
    // }

    fn process_loop(&mut self) {
        log::info!("processing loop");
    }
}

impl Handler for Worker {

    fn request_bin(&mut self, bin: Vec<u8>) {
        log::info!("requesting bin {:?}", bin);
    }

    fn notify_head_lib(&self, head_lib: (u32, u32)) {
        log::info!("notifying head lib {:?}", head_lib);
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    init_logger();

    let handler = Worker::new(web_sys::window().expect("should have a window in this context"));
    let mut ship = Ship::new(handler);
    ship.request_status().expect("Fail to request SHIP Status");

    let ws = WebSocket::new(SHIP_URL)?;
    ws.set_binary_type(BinaryType::Arraybuffer);

    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        let response = e.data();
        log::info!("message event, received data: {:?}", response);
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let cloned_ws = ws.clone();
    let notification: Rc<dyn Fn(WebSocketStatus)> = Rc::new(move |status| match status {
        WebSocketStatus::Opened(e) => {
            log::info!("ws opened: {:?}", e);
            cloned_ws.send_with_str("open ping").unwrap();
        },
        WebSocketStatus::Closed(e) => {
            log::error!("ws closed: {:?}", e.message());
        },
        WebSocketStatus::Error(e) => {
            log::error!("ws error: {}", e.message());
            cloned_ws.send_with_str("error ping").unwrap();
        },
    });

    let notify = notification.clone();
    let listener_open = move |e: &Event| {
        notify(WebSocketStatus::Opened(e.clone()));
    };

    let notify = notification.clone();
    let listener_closed = move |e: &Event| {
        let e = e.dyn_ref::<CloseEvent>().unwrap();
        notify(WebSocketStatus::Closed(e.clone()));
    };

    let notify = notification.clone();
    let listener_error = move |e: &Event| {
        let e = e.dyn_ref::<ErrorEvent>().unwrap();
        notify(WebSocketStatus::Error(e.clone()));
    };

    EventListener::new(&ws, "open", listener_open).forget();
    EventListener::new(&ws, "close", listener_closed).forget();
    EventListener::new(&ws, "error", listener_error).forget();

    Ok(())
}

fn init_logger() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Logging initialized");
}
