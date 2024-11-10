use serde::Deserialize;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub core: CoreConfig,
}

#[derive(Debug, Deserialize)]
pub struct CoreConfig {
    pub gitlab_url: String,
    pub gitlab_projects: Vec<String>,
}

impl Config {
    pub fn new(path: String) -> Self {
        // TODO: Proper error handling comes later
        let config_content = fs::read_to_string(path).unwrap();
        let config: Self = toml::from_str(&config_content).unwrap();
        config
    }
}
