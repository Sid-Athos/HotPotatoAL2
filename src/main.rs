extern crate core;

mod messages;
mod common;
mod challenges;
mod client;
mod error;

use std::env;
use messages::MessageType;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];

}