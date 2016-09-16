pub struct CqlError;

use std::fmt;

impl fmt::Display for CqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cql Gone Awry")
    }
}

impl Error for CqlError {
    fn description(&self) -> {
        "cql gone awry"
    }
}
