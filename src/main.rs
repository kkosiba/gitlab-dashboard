mod action;
mod app;
mod cli;
mod components;
mod config;
mod gitlab;
mod state;
mod tui;

use app::App;
use clap::Parser;
use cli::Cli;
use config::Config;

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let config = Config::new(args.config_file)?;
    let mut app = App::new(config);

    app.run().await?;
    Ok(())
}
