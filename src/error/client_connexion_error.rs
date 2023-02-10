use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct ClientConnexionError;

impl Debug for ClientConnexionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientConnexionError occurred!")
    }
}

impl Display for ClientConnexionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientConnexionError occurred!")
    }
}

impl Error for ClientConnexionError {

}

