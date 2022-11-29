use serde::{Deserialize, Serialize};
use std::fmt::format;

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
                println!("deserialized = {:?}", deserialized);
                return deserialized.unwrap();
            }
            Err(_) => {
                println!("Try again");
                panic!("Le json fournit n'est pas celui d'un message");
            }
        }
    }
}
/*
trait Serializable<S> {
    fn to_json(&self) -> String;

    fn from_str<S>(str: &String) -> S;
}

impl Serializable<Message> for Message {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_str<Message>(&str: String) -> Self {
        let deserialized = serde_json::from_str::<Message>(&str);
        match deserialized {
            Ok(_) => {println!("deserialized = {:?}", deserialized);}
            Err(_) => {println!("Try again");}
        }
    }
}*/

fn main() {
    let message = Message {
        message: "test".to_string(),
    };
    let json_message = message.to_json();
    println!("{json_message}");
}
