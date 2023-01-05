use crate::messages::SubscribeError;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}