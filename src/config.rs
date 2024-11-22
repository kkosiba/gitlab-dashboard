use serde::Deserialize;
use std::{error::Error, fs};
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct Config {
    #[validate(nested)]
    pub core: CoreConfig,
    #[validate(nested)]
    pub ui: UIConfig,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CoreConfig {
    #[validate(url)]
    pub gitlab_url: String,

    #[validate(length(min = 1))]
    pub gitlab_projects: Vec<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UIConfig {
    #[validate(range(min = 1))]
    #[serde(default = "default_max_page_size")]
    pub max_page_size: usize,
}

fn default_max_page_size() -> usize {
    25
}

impl Config {
    pub fn new(path: String) -> Result<Self, Box<dyn Error>> {
        // TODO: Proper error handling comes later
        let config_content = fs::read_to_string(path).unwrap();
        let config: Self = toml::from_str(&config_content).unwrap();
        config.validate()?;
        Ok(config)
    }
}
