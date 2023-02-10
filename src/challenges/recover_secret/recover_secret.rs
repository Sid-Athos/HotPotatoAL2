use crate::challenges::challenge_interface::ChallengeTrait;
use crate::challenges::recover_secret::{RecoverSecretInput, RecoverSecretOutput};

pub struct RecoverSecretChallenge {
    input: RecoverSecretInput,
}

impl ChallengeTrait for RecoverSecretChallenge {
    type Input = RecoverSecretInput;
    type Output = RecoverSecretOutput;

    fn name() -> String {
        return "RecoverSecret".to_string();
    }

    fn new(input: Self::Input) -> Self {
        return RecoverSecretChallenge{
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