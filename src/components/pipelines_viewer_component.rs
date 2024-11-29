use color_eyre::Result;
use layout::Flex;
use ratatui::{prelude::*, widgets::*};
use std::cmp::{max, min};
use std::string::ToString;
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{
    action::Action,
    config::Config,
    gitlab::{fetch_pipelines, PipelineStatus, PipelinesData},
};

#[derive(Default)]
pub struct PipelinesViewerState {
    pub active_project: Option<String>,
    pub active_operation_index: usize,
    pub active_filters: Vec<String>,
    pub active_page: usize, // add 1 to this, as default will make it 0
    pub pipelines_data: PipelinesData,
}

#[derive(Default)]
pub struct PipelinesViewer {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    state: PipelinesViewerState,
}

impl PipelinesViewer {
    pub fn new() -> Self {
        Self::default()
    }

    fn load_pipelines_data(&mut self) {
        match fetch_pipelines(
            self.config.core.gitlab_url.clone(),
            self.state.active_project.clone().unwrap(),
            self.config.ui.max_page_size,
        ) {
            Ok(results) => self.state.pipelines_data = PipelinesData::Loaded(results),
            Err(error) => self.state.pipelines_data = PipelinesData::Errors(error),
        }
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
                self.load_pipelines_data();
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let pipelines = match &self.state.pipelines_data {
            PipelinesData::Loaded(pipelines) => pipelines,
            _ => &vec![], // This arm should never happen TODO: improve this logic later
        };
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
            let hightlight_style = if i == self.state.active_operation_index {
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

        let paginator = build_paginator(pipelines.len(), self.state.active_page + 1);
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
                    self.state.active_project.clone().unwrap(),
                ))
                .title(
                    Line::styled(
                        format!("Filters: {}", &self.state.active_filters.join(", ")),
                        Style::default().add_modifier(Modifier::ITALIC),
                    )
                    .right_aligned(),
                )
                .borders(Borders::ALL)
                .title_bottom(
                    Line::from(format!(
                        "{} of {}",
                        self.state.active_operation_index + 1,
                        pipelines.len()
                    ))
                    .right_aligned(),
                )
                .title_bottom(Line::from(format!("Pages: {}", paginator)).left_aligned()),
        );

        frame.render_widget(table, area);
        Ok(())
    }
}

pub fn build_paginator(total_pages: usize, current_page: usize) -> String {
    let mut pagination = String::new();

    // Add the "previous page" marker
    pagination.push_str("<H ");

    // Add the first page
    if current_page == 1 {
        pagination.push_str("[1] ");
    } else {
        pagination.push_str("1 ");
    }

    // Add ellipsis if needed
    if current_page > 3 {
        pagination.push_str("... ");
    }

    // Add the current page or nearby pages
    for page in max(2, current_page - 1)..=min(total_pages - 1, current_page + 1) {
        if page == current_page {
            pagination.push_str(&format!("[{}] ", page));
        } else {
            pagination.push_str(&format!("{} ", page));
        }
    }

    // Add ellipsis if needed
    if current_page < total_pages - 2 {
        pagination.push_str("... ");
    }

    // Add the last page
    if total_pages > 1 {
        pagination.push_str(&format!("{} ", total_pages));
    }

    // Add the "next page" marker
    pagination.push_str("L>");

    pagination
}
