// #[macro_use] extern crate log;
// use bytes::{Bytes, BytesMut};
use env_logger;
use serde_json::Value;
use websocket::{ClientBuilder, Message, OwnedMessage};

mod serialize;

use eosio_cdt::eos::{eos_deserialize, eos_serialize};
use state_history::{
    GetBlocksRequestV0, GetBlocksResultV0, GetStatusRequestV0, GetStatusResponseV0, ShipRequests,
    ShipResults, SignedBlockHeader, TransactionTraceV0,
};

static ADDRESS: &str = "http://localhost:8080";
static INIT_BLOCK: u32 = 10;
static END_BLOCK: u32 = 12;

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
    print_ship_status(status_message);

    let request_blocks = request_blocks_message();
    client.send_message(&request_blocks).unwrap();

    for message in client.incoming_messages() {
        let message: OwnedMessage = message.unwrap();
        let processed_block = process_block(message);
        if processed_block >= END_BLOCK {
            println!("reached end block, finishing...");
            break;
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
    send_request(ShipRequests::GetStatus(request))
    // let mut buf = BytesMut::new();
    // request.write(&mut buf);
    // let bytes = eos_serialize(&request)
    //     .expect("fail to serialize status request");
    // Message::binary(bytes)
}

fn request_blocks_message<'a>() -> Message<'a> {
    let request = GetBlocksRequestV0 {
        start_block_num: INIT_BLOCK,
        end_block_num: END_BLOCK + 1,
        max_messages_in_flight: 4294967295,
        have_positions: vec![],
        irreversible_only: false,
        fetch_block: true,
        fetch_traces: true,
        fetch_deltas: true,
    };
    send_request(ShipRequests::GetBlocks(request))
    // // let mut buf = BytesMut::new();
    // // request.write(&mut buf);
    // let bytes = eos_serialize(&request)
    //     .expect("fail to serialize blocks request");
    // Message::binary(bytes)
    // Message::binary(buf.to_vec())
}

fn send_request<'a>(request: ShipRequests) -> Message<'a> {
    let bytes = eos_serialize(&request).expect("fail to serialize blocks request");
    Message::binary(bytes)
}

fn handle_response(message: OwnedMessage) -> ShipResults {
    if let OwnedMessage::Binary(bin) = message {
        // let mut bin_bytes = Bytes::from(bin);
        // let status_response = GetStatusResponseV0::read(&mut bin_bytes);
        // Message::binary(bytes)
        let status_response: ShipResults =
            eos_deserialize(&bin).expect("fail to parse state history response");

        // println!("Status response {:?}", status_response);
        status_response
    } else {
        panic!("not a binary message from ship");
    }
}

fn print_ship_status(message: OwnedMessage) {
    match handle_response(message) {
        ShipResults::GetStatus(status_response) => {
            println!("Status response {:?}", status_response)
        }
        _ => panic!("Fail to parse the SHIP status message"),
    }
    // if let OwnedMessage::Binary(bin) = message {
    //     // let mut bin_bytes = Bytes::from(bin);
    //     // let status_response = GetStatusResponseV0::read(&mut bin_bytes);
    //     // Message::binary(bytes)
    //     let status_response: GetStatusResponseV0 = eos_deserialize(&bin)
    //         .expect("fail to parse status response");
    //     println!("Status response {:?}", status_response);
    // } else {
    //     panic!("Fail to parse the SHIP status message");
    // }
}

fn process_block(message: OwnedMessage) -> u32 {
    match handle_response(message) {
        ShipResults::GetBlocks(block_result) => {
            println!("\n>>> {:?}", block_result);
            if let Some(block) = block_result.this_block {

                if let Some(block_bytes) = block_result.block {
                    let block_header: SignedBlockHeader = eos_deserialize(&block_bytes).unwrap();
                    println!("Block Header >>> {:?}", block_header)
                }

                return block.block_num;
            }
        }
        _ => panic!("Fail to parse the SHIP block result"),
    }
    // if let OwnedMessage::Binary(bin) = message {
    //     // let mut bin_bytes = Bytes::from(bin);
    //     // let block_response = GetBlocksResultV0::read(&mut bin_bytes);
    //     let block_result: GetBlocksResultV0 = eos_deserialize(&bin)
    //         .expect("fail to parse block result");
    //     println!("\n>>> {:?}", block_result);

    //     if let Some(block) = block_result.this_block {
    //         return block.block_num;
    //     }
    // }
    0
}
