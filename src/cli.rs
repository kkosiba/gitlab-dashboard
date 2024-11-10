use clap::Parser;

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
