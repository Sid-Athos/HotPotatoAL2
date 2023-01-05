use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}