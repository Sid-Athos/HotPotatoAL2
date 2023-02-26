extern crate core;

mod messages;
mod common;
mod challenges;
mod meh;

use std::borrow::Borrow;
use messages::MessageType;

use crate::challenges::hash_cash::MD5HashCashOutput;

fn main() {
    let buf = "\"Hello\"".to_string();

    let message = MessageType::deserialize_from_string(buf);

    println!("{}", message.to_string());

    println!("{}", hash_cash(12, "Sid".to_string()).hashcode)
}


fn hash_cash(complexity : u32, message: String) -> MD5HashCashOutput {
    let complexity_to_bits_string = (0..complexity).map(|_| "0").collect::<String>();

    println!("{}", complexity_to_bits_string.len());

    return MD5HashCashOutput {
        seed: 0,
        hashcode : message,
    }
}