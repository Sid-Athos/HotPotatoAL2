extern crate core;

mod messages;
mod common;
mod challenges;
mod client;
mod error;

use std::env;
use std::string::ToString;
use serde_json::to_string;
use messages::MessageType;
use crate::client::client::Client;
use crate::error::{ArgumentsError, ClientConnexionError};

const USAGE: &str =
"Usage of client.exe :\n\
\t-ip <server_ip>\n\
\t-pseudo <your_pseudo>\
";

use crate::challenges::hash_cash::MD5HashCashOutput;

fn main() {
    let mut args = env::args();
    let exe = args.next().ok_or(ArgumentsError).expect(USAGE);
    println!("{}", USAGE);
    println!("You launch '{}'", exe);


    let mut ip = "".to_string();
    let mut pseudo = "".to_string();
    while args.len() != 0 {
        let query = args.next().ok_or(ArgumentsError).expect(USAGE);
        let value = args.next().ok_or(ArgumentsError).expect(USAGE);

        println!("query : '{}', value : '{}'", &query, &value);

        match query.as_str() {
            "-ip" => {
                ip = value;
            }
            "-pseudo" => {
                pseudo = value;
            }
            "-help" => {
                println!("{}", USAGE);
            }
            _ => {
                println!("Argument '{}' not implemented, please follow usage.\n", &query);
                println!("{}", USAGE);
            }
        }
    }

    let mut client: Client = Client{
        pseudo,
        leader_board: None,
        server_connected_ip: "none".to_string(),
        game_version: 0,
        is_connected: false,
        stream: None,
        listener: None,
    };
    
    match client.connect(&ip) {
        Ok(_) => { println!("Connection OK") }
        Err(err) => {
            println!("Error on client connection");
            println!("{0}", err);
        }
    }

}