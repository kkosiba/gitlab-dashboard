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

use color_eyre::Result;

fn main() -> Result<()> {
    let args = Cli::parse();
    let config = Config::new(args.config_file)?;
    let mut app = App::new(config);

    app.run()?;
    Ok(())
}
