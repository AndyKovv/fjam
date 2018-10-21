
use user::models::NewUserProfile;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::user_profiles;

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct UserIdExtractor {
    id: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserProfile {
    id: usize,
    name: String,
    second_name: String,
    email: String,
}

impl UserProfile {
    pub fn new() -> Self {
        // Create new instance of user profile
        Self {
            id: 1,
            name: "Andy".to_string(),
            second_name: "Kovv".to_string(), email: "andy_kovv@gmail.com".to_string()
        }

        // To future - Here we can use cache structures.
        // Like get data from cache not from database.
        // Make like adapter for get by id
    }
}

// pub fn create_test_user(name: &str, second_name: &str, email: &str) -> Result<usize, diesel::result::Error> {
//         // let manger = ConnectionManager::new("/var/www/fjam/fjam.db");
//         let conn = PgConnection::establish("/var/www/fjam/fjam.db")
//             .unwrap_or_else(|_| panic!("Error connecting to {}", "/var/www/fjam/fjam.db"));

        

//         let new_user = NewUserProfile {
//             name: name,
//             second_name: second_name,
//         };
//         diesel::insert_into(user_profiles::table).values(&new_user).execute(&conn)
//     }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_should_add_user_to_database() {
    //     let _user_id = create_test_user("Andy", "Kovv", "andy.kovv@gmail.com").map_err(|_| panic!("{:?}", "User not created"));
    //     // assert_eq!(user_id, 1);
    // }
}
