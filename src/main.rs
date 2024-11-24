mod app;
mod cli;
mod config;
mod gitlab;
mod state;
mod ui;

use app::App;
use clap::Parser;
use cli::Cli;
use config::Config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let config = Config::new(args.config_file)?;
    let mut app = App::new(config);

    app.run()?;
    Ok(())
}
