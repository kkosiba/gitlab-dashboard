use clap::Parser;
use serde::Deserialize;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    core: CoreConfig,
}

#[derive(Debug, Deserialize)]
struct CoreConfig {
    gitlab_url: String,
    gitlab_projects: Vec<String>,
}

#[derive(Parser)]
#[command(
    name = "GitLab Dashboard",
    version = "1.0",
    about = "TUI application for monitoring GitLab projects"
)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short = 'c', long = "config-file")]
    pub config_file: String,
}

impl Config {
    pub fn new(path: String) -> Self {
        // TODO: Proper error handling comes later
        let config_content = fs::read_to_string(path).unwrap();
        let config: Self = toml::from_str(&config_content).unwrap();
        config
    }
}
