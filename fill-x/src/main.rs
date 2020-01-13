// #[macro_use] extern crate log;
use env_logger;
use serde_json::Value;
use websocket::{ClientBuilder, Message, OwnedMessage};

mod requests;
mod serialize;

use crate::requests::{AbiDeserializer, AbiSerializer};

static ADDRESS: &str = "http://localhost:8080";

fn main() {
    env_logger::init();
    println!("Initialized logger");

    let mut _client_builder = ClientBuilder::new(ADDRESS).unwrap();
    let mut client = match _client_builder.connect_insecure() {
        Ok(connection) => {
            println!("connected...");
            connection
        }
        Err(error) => {
            eprintln!("Error ocurred: {:?}", error);
            std::process::exit(1);
        }
    };

    let initial_message = client
        .recv_message()
        .expect("Never received the initial SHIP message");
    let ship_abi = init_abi_definitions(&initial_message).unwrap();
    println!(">>> SHIP Abi \n{}", ship_abi);

    let msg = request_blocks_message();
    println!("msg is {:?}", msg);
    client.send_message(&msg).unwrap();
    println!("msg sent to server!");
    for message in client.incoming_messages() {
        let message: OwnedMessage = message.unwrap();
        if let OwnedMessage::Binary(bin) = message {
            println!("Recv Binary: {:?}", bin);
            let response = requests::GetStatusResponse::deserialize(bin);
            println!("Status response {:?}", response);
        }
    }

    client.shutdown().unwrap();
}

fn init_abi_definitions(message: &OwnedMessage) -> Result<Value, &'static str> {
    let text = match message {
        OwnedMessage::Text(t) => t,
        _ => return Err("missing initial message text"),
    };

    let value = match serde_json::from_str(text) {
        Ok(v) => v,
        _ => return Err("fail to parse the state history initial abi"),
    };

    Ok(value)
}

fn request_blocks_message<'a>() -> Message<'a> {
    let request = requests::GetStatusRequest {};
    let request_msg = request.serialize();

    let msg = Message::binary(request_msg);
    // let msg = Message::text(request_json);
    println!("msg created {:?}", msg);
    msg
}
