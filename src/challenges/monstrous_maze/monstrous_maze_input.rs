use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u32,
}