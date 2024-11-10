mod app;
mod cli;
mod config;
mod ui;

use app::{App, Pane};
use clap::Parser;
use cli::Cli;
use config::Config;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io, time::Duration};

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| {
            ui::draw(f, app);
        })?;

        if handle_event(app)? {
            break;
        }
    }
    Ok(())
}

fn handle_event(app: &mut App) -> Result<bool, Box<dyn Error>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('j') => app.next(),
                KeyCode::Char('k') => app.previous(),
                KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.switch_to_left()
                }
                KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.switch_to_right()
                }
                KeyCode::Tab => app.next_tab(),
                KeyCode::BackTab => app.previous_tab(), // BackTab = Shift + Tab
                _ => {}
            }
        }
    }
    Ok(false)
}

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

    let result = run_app(&mut terminal, &mut app);

    // Clean up: Leave alternate screen, disable raw mode, show cursor
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }
    Ok(())
}
