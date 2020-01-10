// #[macro_use] extern crate log;
use env_logger;
use websocket::ClientBuilder;
use websocket::{ OwnedMessage };

static ADDRESS: &str = "http://localhost:8080";

fn main() {
  env_logger::init();
  println!("Initialized logger");

  let mut _client_builder = ClientBuilder::new(ADDRESS).unwrap();
  let mut connection = match _client_builder.connect_insecure() {
    Ok(connection) => {
      println!("connected...");
      connection
    },
    Err(error) => {
      println!("Error ocurred: {:?}", error);
      std::process::exit(-1);
    }
  };


    let message: OwnedMessage = connection.recv_message().unwrap();
    println!("Recv: {:?}", message);

  connection.shutdown().unwrap();
}
