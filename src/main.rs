mod app;
use app::{App, Pane};
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
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn render_pane(
    f: &mut Frame,
    area: Rect,
    lines: &[String],
    selected_index: usize,
    is_active: bool,
    title: &str,
) {
    let width = area.width as usize;

    let text: Vec<Line> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let padded_line = if line.len() < width {
                format!("{:<width$}", line, width = width)
            } else {
                line.clone()
            };

            let style = if i == selected_index && is_active {
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            Line::from(Span::styled(padded_line, style))
        })
        .collect();

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(paragraph, area);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
                .split(size);

            render_pane(
                f,
                chunks[0],
                &app.left_lines,
                app.left_index,
                app.active_pane == Pane::Left,
                "Left Pane",
            );
            render_pane(
                f,
                chunks[1],
                &app.right_lines,
                app.right_index,
                app.active_pane == Pane::Right,
                "Right Pane",
            );
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
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

fn main() -> Result<(), Box<dyn Error>> {
    let left_file = "file.txt"; // TODO: This should eventually load content dynamically
    let right_file = "file.txt"; // TODO: This as well
    let left_lines = read_lines(left_file)?;
    let right_lines = read_lines(right_file)?;

    // Enter alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    let mut app = App::new(left_lines, right_lines);
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

fn read_lines<P>(filename: P) -> Result<Vec<String>, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().map_while(Result::ok).collect())
}
