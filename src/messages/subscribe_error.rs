use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}