use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct MD5HashCashInput {
    // complexity in bits
    pub complexity: u32,
    // message to sign
    pub message: String,
}