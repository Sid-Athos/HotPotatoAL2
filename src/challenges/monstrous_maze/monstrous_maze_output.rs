use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct MonstrousMazeOutput {
    pub path: String,
}