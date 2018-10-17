/// Config module uses for create config for different connections
/// Get configs from for coonectio from here
/// If need update or add new config - add/update tests, add/update code
/// Tests need to be up to date 


use std::env;
use std::path::{Path};
use dotenv::from_path;
use std::collections::HashMap;
use test_utils::files_utils::FileBuilder;
const ENV_FILE_VAR: &'static str = "config_file";

pub struct Config;

impl Config {
    /// Get_postgress_connection_url() get database connection url from config
    /// This method uses only for postgresql connection.
    pub fn parse_env_vars() -> HashMap<String, String> {
        // Method should patse env variables and add it to hash map
        let mut env_variables = HashMap::new();
        for (key, value) in env::vars() {
            env_variables.insert(key, value);
        }
        env_variables
    }

    pub fn check_env_var(key: &str) -> String {
        // Method should check if env var exist
        // Think! need raise error if not found, or add flag with param for panic
        match env::var(key) {
            Ok(val) => val.to_string(),
            Err(_) => {
                error!("Env variable not found {}", key);
                panic!();
            }
        }
    }

    pub fn get_postgress_connection_url() -> String {
        // Dummy hardcoded url
        // connect config later
        let config = Config::parse_env_vars();
        let conf_file_path = match config.get(ENV_FILE_VAR) {
            Some(conf_file) => {
                info!("Config path is set, path: {}", &conf_file);
                conf_file
            },
            None => {
                error!("Config path is not set, please set env variable {}", ENV_FILE_VAR);
                panic!();
            },
        };
        let path = Path::new(conf_file_path);
        from_path(path).ok();

        let check_env_var = Config::check_env_var;
        let user = check_env_var("POSTGRES_USER");
        let pass = check_env_var("POSTGRES_PASS");
        let db = check_env_var("POSTGRES_DATABASE");
        format!("postgres://{}:{}@localhost/{}",user, pass, db )
    }
}


#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_should_create_postgres_url_from_created_file() {
        let data = b"POSTGRES_USER=fjam \n POSTGRES_PASS=fjam \n POSTGRES_DATABASE=fjam";
        let created_file = 
            FileBuilder::new()
            .create_tmp_dir("temp")
            .file_name("env")
            .add_data(data)
            .build()
            .create();
        env::set_var("config_file", created_file.get_file_path());
        let postgres_url = Config::get_postgress_connection_url();
        let expected_url = "postgres://fjam:fjam@localhost/fjam";
        created_file.remove_with_dir();
        assert_eq!(postgres_url, expected_url);
    }
}
