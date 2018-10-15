

pub struct Config;

impl Config {
    /// Get_postgress_connection_url() get database connection url from config
    /// This method uses only for postgresql connection.
    pub fn get_postgress_connection_url() -> String {
        // Dummy hardcoded url
        // connect config later
        "postgres://fjam:fjam@localhost/fjam".to_string()
    }
}


#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_should_return_postgresql_config() {
        let postgres_url = Config::get_postgress_connection_url();
        let expected_url = "postgres://fjam:fjam@localhost/fjam";
        assert_eq!(postgres_url, expected_url);
    }
}
