use serde::{Deserialize, Serialize};

use crate::challenges::hash_cash::MD5HashCashOutput;
use crate::challenges::monstrous_maze::MonstrousMazeOutput;
use crate::challenges::nonogram::NonogramSolverOutput;
use crate::challenges::recover_secret::RecoverSecretOutput;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    MonstrousMaze(MonstrousMazeOutput),
    RecoverSecret(RecoverSecretOutput),
    NonogramSolver(NonogramSolverOutput),
}