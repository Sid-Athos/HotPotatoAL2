use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct NonogramSolverOutput {
    pub grid: String,
}