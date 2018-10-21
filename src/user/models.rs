#![allow(proc_macro_derive_resolution_fallback)]

use schema::{user_profiles, users};

#[derive(Debug, Queryable)]
pub struct User {
    // Queriable
    id: usize,
    email: String,
    password: String,

}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Queryable)]
pub struct UserProfile {
    // Structure should be same as table in database
    id: usize,
    user_id: i32,
    name: String,
    second_name: String,
}

#[derive(Debug, Insertable, Associations, PartialEq)]
#[belongs_to(User)]
#[table_name="user_profiles"]
pub struct NewUserProfile<'a> {
    // Structure for create new user profile in database
    pub user_id: &'a i32,
    pub name: &'a str,
    pub second_name: &'a str,
}
