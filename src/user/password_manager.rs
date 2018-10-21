/// Module for manage passowords for user.
/// Use this for create hash ot verify income passwor

use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};


/// Sructure for manage password.
/// It's implement check password rules, like: length, strong, etc..
#[derive(Debug, Clone)]
pub struct PasswordManager<'a> {
    pub password: &'a str,
    pub hash: Option<String>,
    pub is_valid: bool
}

impl <'a> Default for PasswordManager<'a> {
    fn default() -> Self {
        Self {
            password: "",
            hash: None,
            is_valid: false
        }
    }
}

impl<'a> PasswordManager<'a> {

    fn set_hash(&mut self, hash: String) {
        // Setter for hash attr
        self.hash = Some(hash)
    }

    fn set_is_valid(&mut self, status: bool) {
        // Method should set password validation
        self.is_valid = status;
    }

    pub fn password_is_valid(&self) -> bool {
        // Method should return password validation status
        self.is_valid
    }

    pub fn get_hash(self) -> Option<String> {
        // Method should return Option With hash
        self.hash
    }

    pub fn create_hash(mut self) -> Result<Self, BcryptError> {
        // Method should create hash from income password
         match hash(&self.password, DEFAULT_COST) {
            Ok(hash_data) => {
                self.set_hash(hash_data);
                Ok(self)
            },
            Err(err) => {
                error!("Create password hash failure {:?}", err);
                Err(err)
            }
        }
    }

    pub fn validate_hash(mut self) -> Self {
        // Procedure should validate hash with income password 
        let hash = self.hash
                .clone()
                .unwrap_or("".to_string());
        match verify(&self.password, &hash) {
            Ok(true) => {
                self.set_is_valid(true)
            },
            Ok(false) => {
                error!("Password invalid!");
                self.set_is_valid(false)
            },
            Err(err) => {
                error!("Password validation failed {:?}", err);
                self.set_is_valid(false)
            }
        };
        self
    }
}

pub mod tests_password_manager {
    use super::*;

    #[test]
    fn test_should_create_hash_from_password() {
        let manager = PasswordManager{
            password: "some_password",
            ..Default::default()
        }
        .create_hash()
        .unwrap();
        assert_ne!(
            manager.get_hash(),
            None
        );
    }

    #[test]
    fn test_should_validate_income_password() {
        let manager = PasswordManager {
            password: "SomeShineyPassword",
            ..Default::default()
        }
        .create_hash()
        .unwrap();

        let created_hash = manager.get_hash().unwrap();
        let expected_value = created_hash.clone();
        let validate_passw = PasswordManager {
            password: "SomeShineyPassword",
            hash: Some(created_hash),
            ..Default::default()
        }
        .validate_hash();
        assert_eq!(validate_passw.password_is_valid(), true);

        let invalid_pass = PasswordManager {
            password: "SomeShineyInvalidPass",
            hash: Some(expected_value),
            ..Default::default()
        }
        .validate_hash();
        assert_eq!(invalid_pass.password_is_valid(), false);
    }
}