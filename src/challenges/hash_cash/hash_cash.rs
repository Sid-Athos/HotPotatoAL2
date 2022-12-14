use crate::challenges::challenge_interface::{ChallengeTrait};
use crate::challenges::hash_cash::{MD5HashCashInput, MD5HashCashOutput};


pub struct MD5HashCashChallenge {
    input: MD5HashCashInput,
}

impl ChallengeTrait for MD5HashCashChallenge {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}

