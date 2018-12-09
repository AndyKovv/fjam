/*
    Describe DTO structures here.
    TODO: Think need implement implementations for structures

*/

use convert::{ToMessagePack};
// use serde::Serialize;

/// Structure is using for create new user
#[derive(Debug, Deserialize, Serialize, ToMessagePack)]
pub struct CreateUserDTO {
    pub email: String,
    pub password: String
}
