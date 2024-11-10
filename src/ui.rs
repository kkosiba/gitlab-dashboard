use crate::{app::ApiStatus, App, Pane};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    draw_tabs(f, app, chunks[0]);
    draw_panes(f, app, chunks[1]);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<_> = ["Tab 1", "Tab 2"]
        .iter()
        .cloned()
        .map(|t| Line::from(Span::styled(t, Style::default().fg(Color::White))))
        .collect();

    let tabs = Tabs::new(titles)
        .select(app.active_tab)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn draw_panes(f: &mut Frame, app: &App, area: Rect) {
    let pane_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(area);

    render_pane(f, app, Pane::Left, pane_chunks[0], "Left Pane");
    render_pane(f, app, Pane::Right, pane_chunks[1], "Right Pane");
}

fn render_pane(f: &mut Frame, app: &App, pane: Pane, area: Rect, title: &str) {
    let (lines, index) = match pane {
        Pane::Left => (&*app.api_status_left.lock().unwrap(), app.left_index),
        Pane::Right => (&*app.api_status_right.lock().unwrap(), app.right_index),
    };

    let styled_lines: Vec<Line> = match lines {
        ApiStatus::Loading => vec![Line::from(Span::raw("Loading..."))],
        ApiStatus::Loaded(content) => content
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let style = if i == index && app.active_pane == pane {
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
