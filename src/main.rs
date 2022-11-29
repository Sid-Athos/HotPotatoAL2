mod messages;

use crate::messages::base_message::message_crate::{Message as UseMessage, MessageBehaviourTrait};

fn main() {
    let message = UseMessage::new_from_string("test");
    UseMessage::display_message(&message);
    UseMessage::display_item_type(&message);

    //let deserialized = messages_crate::MessageBehaviour::from_str(&message.message);
    //println!("{item}", item=deserialized.message);
}