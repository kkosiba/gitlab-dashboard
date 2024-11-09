mod app;
use app::{ApiStatus, App, Pane};
mod cli;
use clap::Parser;
use cli::{Cli, Config};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::runtime::Runtime;

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &Arc<Mutex<App>>,
) -> Result<(), Box<dyn Error>> {
    loop {
        // Draw the UI with refactored pane rendering
        terminal.draw(|f| {
            let app = app.lock().unwrap();
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
                .split(size);

            // Render left and right panes
            render_pane(f, &app, Pane::Left, chunks[0]);
            render_pane(f, &app, Pane::Right, chunks[1]);
        })?;

        // Handle keyboard events
        if handle_event(&app)? {
            break;
        }
    }
    Ok(())
}

fn render_pane(f: &mut Frame, app: &App, pane: Pane, area: Rect) {
    let (lines, index, title) = match pane {
        Pane::Left => (
            &*app.api_status_left.lock().unwrap(),
            app.left_index,
            "Left Pane",
        ),
        Pane::Right => (
            &*app.api_status_right.lock().unwrap(),
            app.right_index,
            "Right Pane",
        ),
    };

    let styled_lines: Vec<Line> = match lines {
        ApiStatus::Loading => vec![Line::from(Span::raw("Loading..."))],
        ApiStatus::Loaded(content) => content
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let style = if i == index && &app.active_pane == &pane {
                    Style::default()
                        .fg(Color::Yellow)
                        .bg(Color::Blue)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                Line::from(Span::styled(line.clone(), style))
            })
            .collect(),
    };

    let paragraph =
        Paragraph::new(styled_lines).block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(paragraph, area);
}

fn handle_event(app: &Arc<Mutex<App>>) -> Result<bool, Box<dyn Error>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            let mut app = app.lock().unwrap();
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
                _ => {}
            }
        }
    }
    Ok(false)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let config = Config::new(args.config_file);
    // TODO: Do something with config later
    println!("Loaded config {:?}", config);

    // ratatui boilerplate code
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let app = Arc::new(Mutex::new(App::new()));

    // Spawn threads to fetch data for both panes
    spawn_data_fetch_thread(Arc::clone(&app), Pane::Left, 10);
    spawn_data_fetch_thread(Arc::clone(&app), Pane::Right, 20);

    let result = run_app(&mut terminal, &app);

    // Clean up: Leave alternate screen, disable raw mode, show cursor
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }
    Ok(())
}

fn spawn_data_fetch_thread(app: Arc<Mutex<App>>, pane: Pane, post_limit: usize) {
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let posts = fetch_posts(post_limit)
                .await
                .unwrap_or_else(|_| vec![String::from("Failed to load posts.")]);

            let app = app.lock().unwrap();
            match pane {
                Pane::Left => {
                    *app.api_status_left.lock().unwrap() = ApiStatus::Loaded(posts);
                }
                Pane::Right => {
                    *app.api_status_right.lock().unwrap() = ApiStatus::Loaded(posts);
                }
            }
        });
    });
}

async fn fetch_posts(post_limit: usize) -> Result<Vec<String>, Box<dyn Error>> {
    let url = "https://jsonplaceholder.typicode.com/posts"; // TODO: Make this configurable later
    let response = reqwest::get(url)
        .await?
        .json::<Vec<serde_json::Value>>()
        .await?;

    Ok(response
        .iter()
        .take(post_limit)
        .map(|post| post["title"].as_str().unwrap_or("Untitled").to_string())
        .collect())
}
