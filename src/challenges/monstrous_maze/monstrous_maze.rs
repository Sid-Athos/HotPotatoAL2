use crate::challenges::challenge_interface::ChallengeTrait;
use crate::challenges::monstrous_maze::{MonstrousMazeInput, MonstrousMazeOutput};

pub struct MonstrousMazeChallenge {
    input: MonstrousMazeInput,
}

impl ChallengeTrait for MonstrousMazeChallenge {
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        return "MonstrousMaze".to_string();
    }

    fn new(input: Self::Input) -> Self {
        return MonstrousMazeChallenge{
            input
        }
    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}