mod messages;
mod common;
mod challenges;

use messages::MessageType;

fn main() {
    let buf = "\"Hello\"".to_string();

    let message = MessageType::deserialize_from_string(buf);

    println!("{}", message.to_string());
}