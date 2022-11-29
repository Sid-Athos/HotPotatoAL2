mod messages;
mod challenges;

use crate::messages::message::Message;
use crate::messages::message_interface::MessageInterface;

fn main() {
    let message = Message {
        message: "test".to_string(),
    };

    let json_message = message.to_json();
    println!("{json_message}");

    let deserialized = Message::from_str(&json_message);
    println!("{item}", item=deserialized.message);
}