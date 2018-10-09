
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
