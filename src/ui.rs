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
    render_tabs(f, app, chunks[0]);
    render_main_content(f, app, chunks[1]);
}

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let tab_spans: Vec<Span> = app.tab_titles.iter().map(Span::raw).collect();
    let tabs = Tabs::new(tab_spans)
        .select(app.active_tab)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("GitLab Projects"),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, area);
}

fn render_main_content(f: &mut Frame, app: &App, area: Rect) {
    let pane_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(area);
    render_pane(f, app, Pane::Left, pane_chunks[0], "CI/CD", false);
    render_pane(f, app, Pane::Right, pane_chunks[1], "Details", true);
}

fn render_pane(f: &mut Frame, app: &App, pane: Pane, area: Rect, title: &str, with_sub_tabs: bool) {
    // If with_sub_tabs is true, create a layout with a sub-tabs area at the top
    let (content_area, sub_tabs_area) = if with_sub_tabs {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(area);
        (chunks[1], Some(chunks[0]))
    } else {
        (area, None)
    };

    // Render sub-tabs if present
    if let Some(sub_tabs_area) = sub_tabs_area {
        let sub_tab_labels = vec!["Passed", "Failed", "Cancelled", "Skipped"];
        let sub_tab_spans: Vec<Span> = sub_tab_labels
            .iter()
            .map(|&label| Span::raw(label))
            .collect();
        let sub_tabs = Tabs::new(sub_tab_spans)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_widget(sub_tabs, sub_tabs_area);
    }

    let (data, index) = match pane {
        Pane::Left => (&app.api_status_left, app.left_index),
        Pane::Right => (&app.api_status_right, app.right_index),
    };

    let styled_lines: Vec<Line> = match data {
        ApiStatus::Loading => vec![Line::from(Span::raw("Loading..."))],
        ApiStatus::Loaded(content) => content
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let style = if i == index && app.active_pane == pane {
                    Style::default()
                        .fg(Color::Yellow)
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
    f.render_widget(paragraph, content_area);
}
