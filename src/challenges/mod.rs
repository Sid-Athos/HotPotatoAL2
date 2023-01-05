pub mod hash_cash;
pub mod monstrous_maze;
pub mod nonogram;
pub mod recover_secret;

pub mod challenge;
pub mod challenge_interface;
pub mod challenge_answer;
pub mod challenge_value;
pub mod reported_challenge_result;

pub use challenge::Challenge;
pub use challenge_interface::ChallengeTrait;
pub use challenge_interface::ChallengeInputTrait;
pub use challenge_interface::ChallengeOutputTrait;
pub use challenge_answer::ChallengeAnswer;
pub use challenge_value::ChallengeValue;
pub use reported_challenge_result::ReportedChallengeResult;