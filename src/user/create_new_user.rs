/// Module for create new user in database.
/// Extend this module for add new features or update exist features.

use postgres_connection::pool as pg_pool;
use diesel::prelude::*;
use diesel::dsl::*;
use schema::users::dsl::*;
use schema::users;
use user::models::NewUser;
// use user::errors::{EmailExistError, PasswordCryptError, EmailValidationError};
use fjam_derive::integration_test;
use user::password_manager::PasswordManager;
use user::email_manager::EmailManager;
use common::common_api_errors::{BasicApiError, Errors};
use std::error::Error;

/// New user implementation.
/// Use NewUser for create new user.
/// For the first time -> create user only in database
/// v 0.1.0 -> Implement search usert email in cache, like key:value, where key -> email, value -> password
pub trait NewDBUser {
    fn new<S: Into<String>>(user_email: S, user_password: S) -> Self;
    fn create_new_user(&self) -> Result<i32, Errors>;
}

impl NewDBUser for NewUser {
    /* New database user implementation */

    /// Method should create new user with instance
    fn new<S: Into<String>>(user_email: S, user_password: S) -> Self {
        Self {
            email: user_email.into(),
            password: user_password.into()
        }
    }

    /// Method create new user in database
    /// Use this method only for add (create) new user in postgres database
    fn create_new_user(&self) -> Result<i32, Errors> {
        let mut email_manager = EmailManager::new(&self.email);
        email_manager.exist()?;
        let password_manager = PasswordManager::new(&self.password).create_hash()?;
        let hashed_password = password_manager.get_hash();

        let new_user = NewUser::new(&self.email, &hashed_password);
        let conn = pg_pool().get().unwrap();

        let user_id = insert_into(users)
        .values(&new_user)
        .returning(id)
        .get_results::<(i32)>(&*conn);
        match user_id {
            Ok(ids) => Ok(ids[0]),
            Err(err) => Err(
                Errors::ApiError(BasicApiError::new(err))
            )
        }
    }
}

#[cfg(test)]
pub mod create_new_user_in_database_tests {
    use super::*;

    fn set_up() {
        let conn = pg_pool().get().unwrap();
        let _result = insert_into(users)
        .values((users::email.eq("andy.kovv@gmail.com"), users::password.eq("my_shiney_password")))
        .returning(id)
        .get_results::<(i32)>(&*conn);
    }

    fn tear_down() {
        let conn = pg_pool().get().unwrap();
        conn.execute("DELETE FROM users WHERE id > 0").unwrap();
    }

    #[test]
    #[integration_test]
    fn test_should_create_new_database_user() {
        let new_user_email = "new_shiney_email@gmail.com";
        let new_password = "someShineyPassword";
        let new_db_user: NewUser = NewDBUser::new(new_user_email, new_password);
        let user_id = new_db_user.create_new_user();
        assert_eq!(user_id.is_ok(), true);
    }

    #[test]
    #[integration_test]
    fn test_should_raise_error_on_email_exist() {
        let new_user_email = "andy.kovv@gmail.com";
        let new_password = "someShineyPassword";
        let new_db_user: NewUser = NewDBUser::new(new_user_email, new_password);
        let user = new_db_user.create_new_user();
        assert_eq!(user.is_err(), true);
    }
}

impl NewUser {
/* 
    Think need implementd another implemetation.
    This implementation uses check email method,
    I think need implement it and more generic methods with traits
    So, need think about new implementation((
*/
    pub fn new(u_email: &str, u_password: &str) -> Self {
        Self {
            email: u_email.to_string(),
            password: u_password.to_string()
        }
    }
}