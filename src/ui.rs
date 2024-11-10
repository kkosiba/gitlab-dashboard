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

    let tab_spans: Vec<Span> = app.tab_titles.iter().map(Span::raw).collect();
    let tabs = Tabs::new(tab_spans).select(app.active_tab).block(
        Block::default()
            .borders(Borders::ALL)
            .title("GitLab Projects"),
    );

    f.render_widget(tabs, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let pane_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(chunks[1]);

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
