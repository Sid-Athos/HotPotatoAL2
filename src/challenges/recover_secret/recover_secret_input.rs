use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}