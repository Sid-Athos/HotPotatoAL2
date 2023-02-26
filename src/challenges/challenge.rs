use std::fmt;
use serde::{Deserialize, Serialize};

use crate::challenges::hash_cash::MD5HashCashInput;
use crate::challenges::monstrous_maze::MonstrousMazeInput;
use crate::challenges::nonogram::NonogramSolverInput;
use crate::challenges::recover_secret::RecoverSecretInput;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
    MonstrousMaze(MonstrousMazeInput),
    RecoverSecret(RecoverSecretInput),
    NonogramSolver(NonogramSolverInput),
}

impl fmt::Display for Challenge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}