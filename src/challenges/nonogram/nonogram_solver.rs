use crate::challenges::challenge_interface::ChallengeTrait;
use crate::challenges::nonogram::{NonogramSolverInput, NonogramSolverOutput};

pub struct NonogramChallenge {}

impl ChallengeTrait for NonogramChallenge {
    type Input = NonogramSolverInput;
    type Output = NonogramSolverOutput;

    fn name() -> String {
        return "Nonogram".to_string();
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