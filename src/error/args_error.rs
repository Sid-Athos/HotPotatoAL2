use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct ArgumentsError;

impl Debug for ArgumentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArgumentsError occurred!")
    }
}

impl Display for ArgumentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArgumentsError occurred!")
    }
}

impl Error for ArgumentsError {

}

