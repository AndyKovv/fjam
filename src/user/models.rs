#![allow(proc_macro_derive_resolution_fallback)]

use schema::{user_profiles, users};

#[derive(Debug, Queryable)]
pub struct User {
    // Queriable
    pub id: i32,
    pub email: String,
    pub password: String,

}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
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
pub struct NewUserProfile {
    // Structure for create new user profile in database
    pub user_id: i32,
    pub name: String,
    pub second_name: String,
}
