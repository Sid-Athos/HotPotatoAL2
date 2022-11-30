use crate::challenges::challenge_interface::Challenge;

pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u32,
}

pub struct MonstrousMazeOutput {
    pub path: String,
}

pub struct MonstrousMazeChallenge {}

impl Challenge for MonstrousMazeChallenge {
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        return "MonstrousMaze".to_string();
    }

    fn new(input: Self::Input) -> Self {
        todo!()
    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}