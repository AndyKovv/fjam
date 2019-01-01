/// Module for manage passowords for user.
/// Use this for create hash ot verify income passwor

use bcrypt::{DEFAULT_COST, hash, verify};
use user::errors::{PasswordCryptError, PasswordInvalidError};
use common::common_api_errors::{BasicApiError, HasStatusCode};

/// Sructure for manage password.
/// It's implement check password rules, like: length, strong, etc..
#[derive(Debug, Clone)]
pub struct PasswordManager<'a> {
    pub password: &'a str,
    pub hash: String,
    pub is_valid: bool
}

impl <'a> Default for PasswordManager<'a> {
    fn default() -> Self {
        Self {
            password: "",
            hash: "".to_string(),
            is_valid: false
        }
    }
}

impl<'a> PasswordManager<'a> {

    fn set_hash(&mut self, hash: String) {
        // Setter for hash attr
        self.hash = hash
    }

    fn set_is_valid(&mut self, status: bool) {
        // Method should set password validation
        self.is_valid = status;
    }

    pub fn password_is_valid(&self) -> bool {
        // Method should return password validation status
        self.is_valid
    }

    pub fn get_hash(self) -> String {
        // Method should return Option With hash
        self.hash
    }

    pub fn new(user_password: &'a str) -> Self {
        Self {
            password: user_password,
            ..Default::default()
        }
    }

    pub fn create_hash(mut self) -> Result<Self, Box<BasicApiError>> {
        // Method should create hash from income password
         match hash(&self.password, DEFAULT_COST) {
            Ok(hash_data) => {
                self.set_hash(hash_data);
                Ok(self)
            },
            Err(err) => {
                error!("Create password hash failure {:?}", err);
                Err(BasicApiError::new(PasswordCryptError::crypt_passw_error()))
            }
        }
    }

    pub fn validate_hash(mut self) -> Result<Self, Box<BasicApiError>>{
        // Procedure should validate hash with income password 
        match verify(&self.password, &self.hash) {
            Ok(true) => {
                self.set_is_valid(true);
                Ok(self)
            },
            Ok(false) => {
                error!("Password invalid!");
                self.set_is_valid(false);
                Err(BasicApiError::new(PasswordInvalidError::password_invalid()))
            },
            Err(err) => {
                error!("Password validation failed {:?}", err);
                self.set_is_valid(false);
                Err(BasicApiError::new(PasswordInvalidError::error_password_validation()))
            }
        }
    }
}

pub mod tests_password_manager {
    use super::*;

    #[test]
    fn test_should_create_hash_from_password() {
        let manager = PasswordManager::new(
            "some_password"
        )
        .create_hash()
        .unwrap();
        assert_ne!(
            manager.get_hash(),
            ""
        );
    }

    #[test]
    fn test_should_validate_income_password() {
        let manager = PasswordManager::new(
            "SomeShineyPassword",
        )
        .create_hash()
        .unwrap();

        let created_hash = manager.get_hash();
        let expected_value = created_hash.clone();
        let validate_passw = PasswordManager{
            password: "SomeShineyPassword",
            hash: created_hash,
            ..Default::default()
        }
        .validate_hash().unwrap();
        assert_eq!(validate_passw.password_is_valid(), true);

        let invalid_pass = PasswordManager {
            password: "SomeShineyInvalidPass",
            hash: expected_value,
            ..Default::default()
        }
        .validate_hash().unwrap();
        assert_eq!(invalid_pass.password_is_valid(), false);
    }
}