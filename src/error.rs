use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CqlError {
    Undefined,
    Transport
}

impl fmt::Display for CqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Undefined => write!(f, "undefined funcationality"),
            Transport => write!(f, "transport error"),
        }
    }
}

impl Error for CqlError {
    fn description(&self) -> &str {
        "cql gone awry"
    }
}
