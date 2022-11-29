use std::any::type_name;

pub mod message_crate {
    use std::any::type_name;
    use serde::{Deserialize, Serialize};
    use std::fmt::format;
    use crate::messages::messages_types_structs::message_structs::Message;


    pub trait MessageBehaviourTrait {
        fn new_from_buffer(buf: Vec<u8>) -> Self;

        fn new_from_string(str: &str) -> Self;

        fn display_item_type<T>(item: T);

        fn display_message(message: &Message);

        fn to_json(&self) -> String;

        fn from_str(str: &str) -> Self;
    }
    impl MessageBehaviourTrait for Message {
         fn new_from_buffer(buf: Vec<u8>) -> Message {
            let message = String::from_utf8_lossy(&buf).to_string();
            return Message { message };
        }

        fn new_from_string(str: &str) -> Self {
            let message = str.to_string();
            return Message { message };
        }

        fn display_item_type<T>(item: T) {
            println!("{}",type_name::<T>())
        }

        fn display_message(message: &Message) {
            let message = &message.message;
            println!("{message}")
        }

        fn to_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        fn from_str(str: &str) -> Self {
            let deserialized = serde_json::from_str::<Message>(&str);
            match deserialized {
                Ok(_) => {
                    return deserialized.unwrap();
                }
                Err(_) => {
                    println!("Try again");
                    panic!("Le json fournit n'est pas celui d'un message");
                }
            }
        }
    }

}

