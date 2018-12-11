use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct CommonConvertError {
    // Common error type for convert functional
    details: String,
}

impl CommonConvertError {
    pub fn convert_error() -> Self {
        // Method should create default message for convert errror
        Self {
            details: "Error deserialize data".to_string(),
        }
    }
}

impl Display for CommonConvertError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CommonConvertError {
    fn description(&self) -> &str {
        &self.details
    }
}
