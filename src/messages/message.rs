
use serde::{Deserialize, Serialize};
use crate::messages;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub(crate) message: String,
}

impl messages::message_interface::MessageInterface for Message {
    fn new(buf: Vec<u8>) -> Self {
        let message = String::from_utf8_lossy(&buf).to_string();
        return Message { message };
    }

    fn to_json(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }

    fn from_str(str: &String) -> Self {
        let deserialized = serde_json::from_str::<Self>(&str);
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


