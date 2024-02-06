use std::fs;

use toml::{from_str, to_string};
use crate::config_def::ConfigTOML;

pub fn load_config(path: &str) -> ConfigTOML {
    let config = fs::read_to_string(path).expect("Error reading config file");
    let config: ConfigTOML = from_str(&config).expect("Error parsing config file");
    config
}

pub fn load_creds(config: ConfigTOML) -> (String, String) {
    let username = config.login.username;
    let password = config.login.password;
    (username, password)
}

pub fn save_creds(path: &str, username: &str, password: &str) -> Result<(), String> {
    let mut config = load_config(path);
    config.login = crate::config_def::Login {
        username: username.to_string(),
        password: password.to_string(),
    };
    let config = to_string(&config);
    if config.is_err() {
        return Err(String::from("Error serializing config file"));
    }
    let config = config.unwrap();
    let write = fs::write(path, config);
    if write.is_err() {
        return Err(String::from("Error writing config file"));
    }
    Ok(())
}