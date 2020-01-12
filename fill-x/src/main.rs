// #[macro_use] extern crate log;
use env_logger;
use serde_json::Value;
use websocket::{ClientBuilder, Message, OwnedMessage};

mod serialize;
mod requests;

use crate::requests::AbiSerializer;

static ADDRESS: &str = "http://localhost:8080";
static INITIAL_BLOCK: u32 = 1;

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
    println!("Recv: {:?}", message);
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
  // let request_json = r#"[
  //   "get_blocks_request_v0",
  //   {
  //     "start_block_num": 1,
  //     "end_block_num": 10,
  //     "max_messages_in_flight": 4294967295,
  //     "have_positions": [],
  //     "irreversible_only": false,
  //     "fetch_block": true,
  //     "fetch_traces": true,
  //     "fetch_deltas": true
  //   }
  // ]"#;

  // println!("request json {}", request_json);

  // let request_name = "get_status_request_v0";
  let request = requests::GetStatusRequest{};
  let request_msg = request.serialize();

  let msg = Message::binary(request_msg);
  // let msg = Message::text(request_json);
  println!("msg created {:?}", msg);
  msg
}
