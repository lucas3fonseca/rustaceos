// #[macro_use] extern crate log;
use env_logger;
use serde_json::Value;
use websocket::{ClientBuilder, Message, OwnedMessage};

mod serialize;

use eosio_cdt::eos::{eos_deserialize, eos_serialize};
use state_history::{
    GetBlocksRequestV0, GetStatusRequestV0, ShipRequests, ShipResults, SignedBlockHeader,
    TransactionTraceV0,
};

static ADDRESS: &str = "http://localhost:8080";
static INIT_BLOCK: u32 = 1;
static END_BLOCK: u32 = 999999999;

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
}

fn send_request<'a>(request: ShipRequests) -> Message<'a> {
    let bytes = eos_serialize(&request).expect("fail to serialize blocks request");
    Message::binary(bytes)
}

fn handle_response(message: OwnedMessage) -> ShipResults {
    if let OwnedMessage::Binary(bin) = message {
        let status_response: ShipResults =
            eos_deserialize(&bin).expect("fail to parse state history response");
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
}

fn process_block(message: OwnedMessage) -> u32 {
    match handle_response(message) {
        ShipResults::GetBlocks(block_result) => {
            println!("\n>>> Chain Head: {}", block_result.head.block_num);
            if let Some(block) = block_result.this_block {
                if let Some(block_bytes) = block_result.block {
                    let block_header: SignedBlockHeader =
                        eos_deserialize(&block_bytes).expect("fail to parse signed block header");
                    println!("Block Header >>> {:?}", block_header)
                }

                if let Some(trace_bytes) = block_result.traces {
                    let trace: TransactionTraceV0 =
                        eos_deserialize(&trace_bytes).expect("fail to parse block traces");
                    println!("Trx Traces >>> {:?}", trace);
                }

                return block.block_num;
            }
        }
        _ => panic!("Fail to parse the SHIP block result"),
    }
    0
}
