
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
    name: String,
    second_name: String,
    email: String,
}
