mod messages;
mod common;
mod challenges;
mod client;

use std::env;
use messages::MessageType;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    let buf = "\"Hello\"".to_string();

    let message = MessageType::deserialize_from_string(&buf);

    println!("{}", message.to_string());
}