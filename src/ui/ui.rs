use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Row, Table},
    Frame,
};

use crate::{
    gitlab::Pipeline,
    state::{PipelinesData, State},
};

use super::paginator::build_paginator;

pub fn centered_layout(area: Rect) -> Rect {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(40), // Space above the widget
                Constraint::Percentage(20), // Space for the widget
                Constraint::Percentage(40), // Space below the widget
            ]
            .as_ref(),
        )
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(40), // Space to the left of the widget
                Constraint::Percentage(20), // Space for the widget
                Constraint::Percentage(40), // Space to the right of the widget
            ]
            .as_ref(),
        )
        .split(vertical_chunks[1]);
    horizontal_chunks[1]
}

pub fn render_project_selector(f: &mut Frame, state: &State, projects: &[String]) {
    let area = centered_layout(f.area());
    let list_items: Vec<ListItem> = projects.iter().map(|i| ListItem::new(i.clone())).collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.active_operation_index));

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("GitLab Project Selector")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_loading_view(f: &mut Frame, state: &State) {
    let area = f.area();
    let loading_message = vec![Line::from(Span::styled(
        "Loading...",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ))];
    let block = Paragraph::new(loading_message).alignment(Alignment::Center);
    f.render_widget(block, area);
}

fn render_loaded_view(f: &mut Frame, state: &State, pipelines: &[Pipeline]) {
    let area = f.area();
    let rows = pipelines.iter().enumerate().map(|(i, pipeline)| {
        let style = if i == state.active_operation_index {
            Style::default().fg(Color::Black).bg(Color::White)
        } else {
            Style::default()
        };
        Row::new(vec![
            Span::raw(pipeline.id.to_string()),
            Span::raw(pipeline.status.to_string()),
            Span::raw(pipeline.source.to_string()),
        ])
        .style(style)
    });

    let paginator = build_paginator(pipelines.len(), state.active_page);
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(15),
            Constraint::Percentage(20),
            Constraint::Percentage(65),
        ],
    )
    .block(
        Block::default()
            .title(format!(
                "Pipelines for '{}'",
                state.active_project.clone().unwrap(),
            ))
            .title(
                Line::styled(
                    format!("Filters: {}", &state.active_filters.join(", ")),
                    Style::default().add_modifier(Modifier::ITALIC),
                )
                .right_aligned(),
            )
            .borders(Borders::ALL)
            .title_bottom(
                Line::from(format!(
                    "{} of {}",
                    state.active_operation_index + 1,
                    pipelines.len()
                ))
                .right_aligned(),
            )
            .title_bottom(Line::from(format!("Pages: {}", paginator)).left_aligned()),
    );

    f.render_widget(table, area);
}

fn render_errors_view(f: &mut Frame, state: &State, errors: &[String]) {
    unimplemented!("Implement me!")
}

pub fn render_pipelines_view(f: &mut Frame, state: &State) {
    match &state.pipelines_data {
        PipelinesData::Loading => render_loading_view(f, state),
        PipelinesData::Loaded(pipelines) => render_loaded_view(f, state, pipelines),
        PipelinesData::Errors(errors) => render_errors_view(f, state, errors),
    }
}
