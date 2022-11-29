use messages;

fn main() {
    let message = messages::base_message::Message {
        message: "test".to_string(),
    };

    let json_message = message.to_json();
    println!("{json_message}");

    let deserialized = messages::base_message::Message::from_str(&json_message);
    println!("{item}", item=deserialized.message);
}