
use schema::user_profiles;

#[derive(Debug, Queryable)]
pub struct DbUserProfile {
    // Structure should be same as table in database
    id: usize,
    name: String,
    second_name: String,
    email: String,
}

#[derive(Insertable)]
#[table_name="user_profiles"]
pub struct NewUserProfile {
    // Structure for create new user profile in database
    pub name: String,
    pub second_name: String,
    pub email: String,
}
