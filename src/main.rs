mod app;
use app::{ApiStatus, App, Pane};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    error::Error,
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::runtime::Runtime;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &Arc<Mutex<App>>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let app = app.lock().unwrap();
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);

            let left_lines = match &*app.api_status_left.lock().unwrap() {
                ApiStatus::Loading => vec![Line::from(Span::raw("Loading..."))],
                ApiStatus::Loaded(lines) => lines
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        let style = if i == app.left_index && matches!(app.active_pane, Pane::Left)
                        {
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

            let paragraph = Paragraph::new(left_lines)
                .block(Block::default().borders(Borders::ALL).title("Left Pane"));
            f.render_widget(paragraph, chunks[0]);

            let right_lines = match &*app.api_status_right.lock().unwrap() {
                ApiStatus::Loading => vec![Line::from(Span::raw("Loading..."))],
                ApiStatus::Loaded(lines) => lines
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        let style =
                            if i == app.right_index && matches!(app.active_pane, Pane::Right) {
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
            let paragraph = Paragraph::new(right_lines)
                .block(Block::default().borders(Borders::ALL).title("Right Pane"));
            f.render_widget(paragraph, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let mut app = app.lock().unwrap();
                match key.code {
                    KeyCode::Char('q') => break,
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
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ratatui boilerplate code
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let app = Arc::new(Mutex::new(App::new()));
    let app_clone_left = Arc::clone(&app);
    let app_clone_right = Arc::clone(&app);

    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let posts = fetch_posts(10)
                .await
                .unwrap_or_else(|_| vec![String::from("Failed to load posts.")]);
            *app_clone_left
                .lock()
                .unwrap()
                .api_status_left
                .lock()
                .unwrap() = ApiStatus::Loaded(posts);
        });
    });
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let posts = fetch_posts(20)
                .await
                .unwrap_or_else(|_| vec![String::from("Failed to load posts.")]);
            *app_clone_right
                .lock()
                .unwrap()
                .api_status_right
                .lock()
                .unwrap() = ApiStatus::Loaded(posts);
        });
    });
    let result = run_app(&mut terminal, &app);

    // More ratatui boilerplate
    // Clean up: Leave alternate screen, disable raw mode, show cursor
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }
    Ok(())
}

async fn fetch_posts(num: usize) -> Result<Vec<String>, reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/posts"; // TODO: Make this configurable later
    let response = reqwest::get(url)
        .await?
        .json::<Vec<serde_json::Value>>()
        .await?;

    Ok(response
        .iter()
        .take(num)
        .map(|post| post["title"].as_str().unwrap_or("Untitled").to_string())
        .collect())
}
