/// Module for create errors which belong to user isstance
/// Add errors here for prevent unexpected error implementation 

use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use hyper::{StatusCode};
use common::common_api_errors::HasStatusCode;

impl HasStatusCode for EmailExistError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl HasStatusCode for PasswordCryptError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl HasStatusCode for EmailValidationError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl HasStatusCode for PasswordInvalidError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl HasStatusCode for std::error::Error + 'static  {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

#[derive(Debug)]
pub struct EmailExistError {
    // Error raise when email exist
    details: String,
}

impl EmailExistError {
    pub fn email_exist() -> Self {
        // Method should create default message for email exist
        Self {
            details: "Email already exist".to_string(),
        }
    }
}


impl Display for EmailExistError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for EmailExistError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct PasswordCryptError {
    details: String
}

impl PasswordCryptError {
    pub fn crypt_passw_error() -> Self {
        Self {
            details: "Error crypt password".to_string()
        }
    }
}

impl Display for PasswordCryptError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PasswordCryptError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct EmailValidationError {
    /* Error structure for vaidate email with regex */
    details: String
}

impl EmailValidationError {
    pub fn validation_error() -> Self {
        /* Remove this, move to details method*/
        Self {
            details: "Email validation fail".to_string()
        }
    }
}
impl Display for EmailValidationError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}
impl Error for EmailValidationError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct PasswordInvalidError {
    details: String
}

impl PasswordInvalidError {
    /// Method add text for password invalid error
    pub fn password_invalid() -> Self {
        Self {
            details: "Password invalid!".to_string()
        }
    }

    pub fn error_password_validation() -> Self {
        Self {
            details: "Error while password validation".to_string()
        }
    }
}

impl Display for PasswordInvalidError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PasswordInvalidError {
    fn description(&self) -> &str {
        &self.details
    }
}


#[derive(Debug)]
pub enum EmailManagerErrors {
    EmailValidation(EmailValidationError),
    EmailExist(EmailExistError),
}


impl Display for EmailManagerErrors {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            EmailManagerErrors::EmailValidation(ref err) => err.fmt(f),
            EmailManagerErrors::EmailExist(ref err) => err.fmt(f),
        }
    }
}

impl Error for EmailManagerErrors {
    fn description(&self) -> &str {
        match *self {
            EmailManagerErrors::EmailValidation(ref e) => e.description(),
            EmailManagerErrors::EmailExist(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            EmailManagerErrors::EmailValidation(ref e) => Some(e),
            EmailManagerErrors::EmailExist(ref e) => Some(e),
        }
    }
}

impl From<EmailValidationError> for EmailManagerErrors {
    fn from(e: EmailValidationError) -> Self {
        EmailManagerErrors::EmailValidation(e)
    }
}

impl From<EmailExistError> for EmailManagerErrors {
    fn from(e: EmailExistError) -> Self {
        EmailManagerErrors::EmailExist(e)
    }
}



