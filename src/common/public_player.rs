use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64,
}