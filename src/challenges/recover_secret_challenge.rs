use crate::challenges::challenge_interface::Challenge;

pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

pub struct RecoverSecretChallenge {}

impl Challenge for RecoverSecretChallenge {
    type Input = RecoverSecretInput;
    type Output = RecoverSecretOutput;

    fn name() -> String {
        return "RecoverSecret".to_string();
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