// #[macro_use] extern crate log;
use bytes::BytesMut;
use env_logger;
use serde_json::Value;
use websocket::{ClientBuilder, Message, OwnedMessage};

mod serialize;

use abieos::{AbiDeserializer, AbiSerializer};
use state_history::{
    GetBlocksRequestV0, GetBlocksResultV0, GetStatusRequestV0, GetStatusResponseV0,
};

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

    let request_status = request_status_message();
    client.send_message(&request_status).unwrap();
    let status_message = client
        .recv_message()
        .expect("Never received the SHIP status message");
    print_ship_status(&status_message);

    let request_blocks = request_blocks_message();
    client.send_message(&request_blocks).unwrap();

    let mut blocks = 0;
    for message in client.incoming_messages() {
        let message: OwnedMessage = message.unwrap();
        if let OwnedMessage::Binary(bin) = message {
            let mut bin_bytes = BytesMut::from(&bin[..]);
            let block_response = GetBlocksResultV0::deserialize(&mut bin_bytes);
            println!("\n>>> {:?}", block_response);
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

fn request_status_message<'a>() -> Message<'a> {
    let request = GetStatusRequestV0 {};
    Message::binary(request.serialize())
}

fn request_blocks_message<'a>() -> Message<'a> {
    let request = GetBlocksRequestV0 {
        start_block_num: 1,
        end_block_num: 4294967295,
        max_messages_in_flight: 4294967295,
        have_positions: vec![],
        irreversible_only: false,
        fetch_block: true,
        fetch_traces: true,
        fetch_deltas: true,
    };
    Message::binary(request.serialize())
}

fn print_ship_status(message: &OwnedMessage) {
    if let OwnedMessage::Binary(bin) = message {
        let mut bin_bytes = BytesMut::from(&bin[..]);
        let status_response = GetStatusResponseV0::deserialize(&mut bin_bytes);
        println!("Status response {:?}", status_response);
    } else {
        panic!("Fail to parse the SHIP status message");
    }
}
