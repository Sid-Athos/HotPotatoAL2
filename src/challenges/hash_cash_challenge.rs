use crate::challenges::challenge_interface::Challenge;

pub struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}

pub struct MD5HashCashChallenge {}

impl Challenge for MD5HashCashChallenge {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        return "HashCash".to_string();
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

