use serde::Deserialize;
use std::{fs, process};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub core: CoreConfig,
    pub ui: UIConfig,
}

#[derive(Debug, Deserialize)]
pub struct CoreConfig {
    pub gitlab_url: String,
    pub gitlab_projects: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UIConfig {
    pub max_page_size: usize,
}

impl Config {
    pub fn new(path: String) -> Self {
        // TODO: Proper error handling comes later
        let config_content = fs::read_to_string(path).unwrap();
        let config: Self = toml::from_str(&config_content).unwrap();

        // Validation
        let mut errors: Vec<&str> = vec![];
        // More sophisticated URL validation could be done here, but this will do for now.
        if !(config.core.gitlab_url.starts_with("http://")
            || config.core.gitlab_url.starts_with("https://"))
        {
            errors.push("Config 'core.gitlab_url' must be a valid URL");
        }
        if config.core.gitlab_projects.is_empty() {
            errors.push("Config 'core.gitlab_projects' needs to define at least one project");
        }
        if !errors.is_empty() {
            eprintln!("Invalid config. Errors:\n{}", errors.join("\n"));
            process::exit(1);
        }
        config
    }
}
