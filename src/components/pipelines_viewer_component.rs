use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct PipelinesViewer {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl PipelinesViewer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for PipelinesViewer {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        frame.render_widget(Paragraph::new("hello world"), area);
        Ok(())
    }
}

pub fn render_pipelines_view(f: &mut Frame, state: &State) {
    match &state.pipelines_data {
        PipelinesData::Loading => render_loading_view(f),
        PipelinesData::Loaded(pipelines) => render_loaded_view(f, state, pipelines),
        PipelinesData::Errors(error) => render_errors_view(f, error),
    }
}

fn render_loaded_view(f: &mut Frame, state: &State, pipelines: &[GitlabPipeline]) {
    let area = f.area();
    let header_row = vec![
        "ID",
        "Status",
        "Source",
        "Ref",
        "Created at",
        "Updated at",
        "URL",
    ]
    .into_iter()
    .map(|e| Span::styled(e, Style::default().bold()))
    .collect();

    let rows = pipelines.iter().enumerate().map(|(i, pipeline)| {
        let hightlight_style = if i == state.active_operation_index {
            Style::default().fg(Color::Black).bg(Color::LightYellow)
        } else {
            Style::default()
        };
        let status_style = match pipeline.status {
            PipelineStatus::Failed => Style::default().red(),
            PipelineStatus::Success => Style::default().green(),
            PipelineStatus::Running => Style::default().italic(),
            _ => Style::default(),
        };
        Row::new(vec![
            Span::raw(pipeline.id.to_string()),
            Span::styled(pipeline.status.to_string(), status_style),
            Span::raw(pipeline.source.to_string()),
            Span::raw(&pipeline.git_ref),
            Span::raw(pipeline.created_at.format("%Y-%m-%d %H:%M:%S").to_string()),
            Span::raw(pipeline.updated_at.format("%Y-%m-%d %H:%M:%S").to_string()),
            // TODO: Display URL in a pop-up with details, together with other data
            // Span::raw(&pipeline.web_url),
        ])
        .style(hightlight_style)
    });

    let paginator = build_paginator(pipelines.len(), state.active_page);
    let table = Table::new(
        rows,
        // TODO: Display URL in a pop-up with details, together with other data
        vec![
            Constraint::Length(20), // ID
            Constraint::Length(20), // status
            Constraint::Length(30), // source
            Constraint::Min(30),    // ref
            Constraint::Min(20),    // created at
            Constraint::Min(20),    // updated at
        ],
    )
    .column_spacing(2)
    .header(Row::from(header_row))
    .flex(Flex::SpaceAround)
    .block(
        Block::default()
            .padding(Padding::uniform(1))
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
