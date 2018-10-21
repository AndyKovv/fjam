/// Module for create new user in database.
/// Extend this module for add new features or update exist features.


use user::models::{User, NewUser};

impl <'a> NewUser<'a> {

    pub fn new(email: &'a str, password: &'a str) -> Self {
        Self {
            email: email,
            password: password
        }
    }

    pub fn check_email(self) -> Self {
        // Method should check email for unique

        self

    }

    pub fn crypt_password(self) -> Self {
        // Method should crypt password
        self
    }

    pub fn save_to_db(self) -> Self {
        // Method should add user to database
        self
    }
}

#[cfg(test)]
pub mod tests_user_create {
    use super::*;

    #[test]
    fn test_should_create_new_user_in_database() {
        let email = "andy.kovv@gmail.com";
        let password = "password";
        let _user = NewUser::new(email, password)
            .check_email()
            .crypt_password()
            .save_to_db();
        assert_eq!(1, 1);
    }
}