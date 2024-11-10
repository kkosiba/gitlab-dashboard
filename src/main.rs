mod app;
mod cli;
mod config;

use app::App;
use clap::Parser;
use cli::Cli;
use config::Config;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let config = Config::new(args.config_file);
    let tab_titles = config.core.gitlab_projects.clone();

    // ratatui boilerplate code
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new(tab_titles);
    let result = app.run(&mut terminal);

    // Clean up: Leave alternate screen, disable raw mode, show cursor
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }
    Ok(())
}
