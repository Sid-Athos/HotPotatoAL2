use crate::challenges::challenge_interface::Challenge;

pub struct NonogramInput {
    pub rows: Vec<Vec<u32>>,
    pub cols: Vec<Vec<u32>>,
}

pub struct NonogramOutput {
    pub grid: String,
}

pub struct NonogramChallenge {}

impl Challenge for NonogramChallenge {
    type Input = NonogramInput;
    type Output = NonogramOutput;

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