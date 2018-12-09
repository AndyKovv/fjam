use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Errors {
    ApiError(BasicApiError)
}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Errors::ApiError(ref err) => err.fmt(f),
        }
    }
}

impl Error for Errors {
    fn description(&self) -> &str {
        match *self {
            Errors::ApiError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            Errors::ApiError(ref err) => Some(err),
        }
    }
}

impl From<BasicApiError> for Errors {
    fn from(err: BasicApiError) -> Self {
        Errors::ApiError(err)
    }
}

#[derive(Debug)]
pub struct BasicApiError {
    details: String,
}

impl BasicApiError {
    // Need implement here to json method think
    pub fn new<E: Error>(handler: E) -> Self {
        Self {
            details: handler.description().to_string()
        }
    }

    // pub fn description(&self) -> &str {
    //     &self.details
    // }
}

impl Display for BasicApiError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for BasicApiError {
    fn description(&self) -> &str {
        &self.details
    }
}