use serde::Deserialize;
use std::{fs, process};

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

        // Validation
        let mut errors: Vec<&str> = vec![];
        // More sophisticated URL validation could be done here, but this will do for now.
        if !config.core.gitlab_url.starts_with("http://")
            || !config.core.gitlab_url.starts_with("https://")
        {
            errors.push("Config 'core.gitlab_url' must be a valid URL");
        }
        if config.core.gitlab_projects.len() == 0 {
            errors.push("Config 'core.gitlab_projects' needs to define at least one project");
        }
        if errors.len() > 0 {
            eprintln!("Invalid config. Errors:\n{}", errors.join("\n"));
            process::exit(1);
        }
        config
    }
}
