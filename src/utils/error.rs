//! Basic Custom Error Type for functions within this library.
//! This Custom Error Type contains impl's for Debug, Display and From
//! To use this implement the custom Error type in your functions.
use std::{error, fmt, io, num};

#[derive(Debug)]
pub enum RainstashError {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
}

impl From<io::Error> for RainstashError {
    fn from(err: io::Error) -> RainstashError {
        RainstashError::Io(err)
    }
}

impl From<num::ParseIntError> for RainstashError {
    fn from(err: num::ParseIntError) -> RainstashError {
        RainstashError::ParseInt(err)
    }
}

impl From<reqwest::Error> for RainstashError {
    fn from(err: reqwest::Error) -> RainstashError {
        RainstashError::Reqwest(err)
    }
}

impl From<serde_json::Error> for RainstashError {
    fn from(err: serde_json::Error) -> RainstashError {
        RainstashError::Serde(err)
    }
}

impl fmt::Display for RainstashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RainstashError::Io(ref err) => write!(f, "[ Rainstash Error ] - IO error: {}", err),
            RainstashError::ParseInt(ref err) => {
                write!(f, "[ Rainstash Error ] - ParseInt error: {}", err)
            }
            RainstashError::Reqwest(ref err) => {
                write!(f, "[ Rainstash Error ] - Reqwest error: {}", err)
            }
            RainstashError::Serde(ref err) => {
                write!(f, "[ Rainstash Error ] - Serde error: {}", err)
            }
        }
    }
}

impl error::Error for RainstashError {
    fn description(&self) -> &str {
        match *self {
            RainstashError::Io(ref err) => err.description(),
            RainstashError::ParseInt(ref err) => err.description(),
            RainstashError::Reqwest(ref err) => err.description(),
            RainstashError::Serde(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RainstashError::Io(ref err) => Some(err),
            RainstashError::ParseInt(ref err) => Some(err),
            RainstashError::Reqwest(ref err) => Some(err),
            RainstashError::Serde(ref err) => Some(err),
        }
    }
}
