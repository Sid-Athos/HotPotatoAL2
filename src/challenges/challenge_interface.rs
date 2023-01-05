pub trait ChallengeInputTrait {
    // Create message based on JSON format
    //fn from_str(str: &String) -> Self;
}

pub trait ChallengeOutputTrait {
    // return the string JSON format of this object
    //fn to_json(&self) -> String;
}

pub trait ChallengeTrait {
    /// Données en entrée du challenge
    type Input;

    /// Données en sortie du challenge
    type Output;

    /// Nom du challenge
    fn name() -> String;

    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;

    /// Résout le challenge
    fn solve(&self) -> Self::Output;

    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool;

}