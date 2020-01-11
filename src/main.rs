// #[macro_use] extern crate log;
use env_logger;
use websocket::ClientBuilder;
use websocket::{ OwnedMessage, Message };

mod serialize;

static ADDRESS: &str = "http://localhost:8080";

fn main() {
  env_logger::init();
  println!("Initialized logger");

  let mut _client_builder = ClientBuilder::new(ADDRESS).unwrap();
  let mut client = match _client_builder.connect_insecure() {
    Ok(connection) => {
      println!("connected...");
      connection
    },
    Err(error) => {
      println!("Error ocurred: {:?}", error);
      std::process::exit(-1);
    }
  };


  let message: OwnedMessage = client.recv_message().unwrap();
  println!("Recv: {:?}", message);

  client.send_message(&Message::text("['get_status_request_v0', {}]")).unwrap();

  // for message in client.incoming_messages() {
  //   let message: OwnedMessage = message.unwrap();
  //   println!("Recv: {:?}", message);
  // }

  client.shutdown().unwrap();
}
