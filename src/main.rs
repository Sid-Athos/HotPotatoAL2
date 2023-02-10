extern crate core;

mod messages;
mod common;
mod challenges;
mod client;
mod error;

use std::env;
use messages::MessageType;
use crate::client::client::Client;
use crate::error::ClientConnexionError;

fn main() {
    let args: Vec<String> = env::args().collect();

    //let query = &args[1];
    
    let mut client: Client = Client{
        server_connected_ip: "".to_string(),
        game_version: 1,
        is_connected: false,
        stream: None,
        listener: None,
    };
    
    match client.connect("127.0.0.1:7878".to_string()) {
        Ok(_) => { println!("Connection OK") }
        Err(_) => { println!("Error when trying to connect") }
    }
    
}