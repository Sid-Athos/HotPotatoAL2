use serde::{Deserialize, Serialize};

use crate::challenges::ChallengeValue;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue,
}