//! This module is email manager
//! All logic with email can be added here
//! If need add new implementation or new train add it here

use diesel::prelude::*;
use diesel::dsl::*;
use schema::users::dsl::*;
use schema::users;
use fjam_derive::integration_test;
use postgres_connection::pool;
use regex::Regex;
use user::errors::{EmailValidationError, EmailExistError};
use common::common_api_errors::{BasicApiError};

lazy_static! {
    // Perform a simple regex match to validate email
    static ref EMAIL_REGEX: regex::Regex = Regex::new("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
}

#[derive(Debug)]
pub struct EmailManager {
    /* Email manager struct use for validate email and check its in database */
    pub email: String,
    pub is_valid: bool,
    pub exist: bool
}

impl EmailManager {
    /*
        Email manager use for check email exist
        or validate email by regex
    */

    pub fn new(raw_email: &str) -> Self {
        Self{
            email: raw_email.to_string(),
            ..Default::default()
        }
    }

    pub fn valid(&mut self) -> Result<bool, Box<BasicApiError>> {
        /* Method should validate income email via regex*/
        match EMAIL_REGEX.is_match(&self.email) {
            true => {
                self.set_email_is_valid();
                Ok(true)
            },
            false => Err(BasicApiError::new(
                EmailValidationError::validation_error())
            )
        }
    }

    pub fn exist(&mut self) -> Result<bool, Box<BasicApiError>> {
        /* Method should check email exist in database */
        self.valid()?;
        let conn = pool().get().unwrap();
        let email_exist = users::table
        .filter(users::email.eq(self.email.to_lowercase()))
        .execute(&*conn);
        match email_exist {
            Ok(0) =>  Ok(false),
            _ => {
                self.set_email_exist();
                Err(BasicApiError::new(
                    EmailExistError::email_exist())
                )
            },
        }
    }

    fn set_email_exist(&mut self) {
        /* Method should set email exit */
        self.exist = true;
    }

    fn set_email_is_valid(&mut self) {
        /* Method should set email is valid  */
        self.is_valid = true
    }
}

impl Default for EmailManager {
    fn default() -> Self {
        Self {
            email: "".to_string(),
            is_valid: false,
            exist: false
        }
    }
}

#[cfg(test)]
pub mod mail_manager_tests {
    use super::*;

    /* 
        Maybe need implemet state holder with constant for test?
        I mean, when we create something we need owerride some default values.
        Such as user_name, email, or etc.
        Need think about this!
    */

    fn set_up() {
        let conn = pool().get().unwrap();
        let _result = insert_into(users)
        .values((users::email.eq("andy.kovv@gmail.com"), users::password.eq("my_shiney_password")))
        .returning(id)
        .get_results::<(i32)>(&*conn);
    }

    fn tear_down() {
        let conn = pool().get().unwrap();
        conn.execute("DELETE FROM users WHERE id > 0").unwrap();
    }

    #[test]
    #[integration_test]
    fn test_should_check_email_exist_in_database() {
        let user_email = "andy.kovalev@email.com";
        let mut email_manager = EmailManager::new(user_email);
        assert_eq!(email_manager.exist().unwrap(), false);

        let user_email_extist = "andy.kovv@gmail.com";
        let mut email_manager = EmailManager::new(user_email_extist);
        assert_eq!(email_manager.exist().is_err(), true);
    }

    #[test]
    #[integration_test]
    fn test_should_validate_income_email() {
        let invalid_email = "email@-example.com";
        let mut email_manager = EmailManager::new(invalid_email);
        assert_eq!(email_manager.valid().is_err(), true);
        let valid_email = "andy.kovv@gmail.com";
        let mut email_manager = EmailManager::new(valid_email);
        assert_eq!(email_manager.valid().unwrap(), true);
    }

    #[test]
    #[integration_test]
    fn test_should_check_status_in_structure() {
        let user_email = "som@mail.com";
        let mut manager = EmailManager::new(user_email);
        manager.exist();
        assert_eq!(manager.is_valid, true);
        assert_eq!(manager.exist, false);
    }
}