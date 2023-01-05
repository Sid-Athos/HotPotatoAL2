use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}