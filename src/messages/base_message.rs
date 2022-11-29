use serde::{Deserialize, Serialize};
use std::fmt::format;

pub fn base_message() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Message {
        message: String,
    }

    impl Message {
        fn new(buf: Vec<u8>) -> Self {
            let message = String::from_utf8_lossy(&buf).to_string();
            return Message { message };
        }

        fn to_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        fn from_str(str: &String) -> Self {
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
