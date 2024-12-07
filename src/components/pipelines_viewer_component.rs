use color_eyre::eyre::Error;
use color_eyre::Result;
use layout::Flex;
use ratatui::{prelude::*, widgets::*};
use std::cmp::{max, min};
use std::string::ToString;
use tokio::sync::mpsc::UnboundedSender;

use super::utils::{get_block, prepare_layout, Body, Element};
use super::Component;
use crate::state::State;
use crate::{
    action::Action,
    config::Config,
    gitlab::{fetch_pipelines, PipelineStatus, PipelinesData},
};

#[derive(Default)]
pub struct PipelinesViewerComponent {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    pub active_operation_index: usize,
    pub active_filters: Vec<String>,
    pub active_page: usize, // add 1 to this, as default will make it 0
    pub pipelines_data: PipelinesData,
    show_details_popup: bool,
}

impl PipelinesViewerComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn load_pipelines_data(&mut self, state: &State) {
        match &state.active_gitlab_project {
            Some(gitlab_project) => {
                match fetch_pipelines(
                    self.config.core.gitlab_url.clone(),
                    gitlab_project.clone(),
                    self.config.ui.max_page_size,
                ) {
                    Ok(results) => self.pipelines_data = PipelinesData::Loaded(results),
                    Err(error) => self.pipelines_data = PipelinesData::Errors(error),
                }
            }
            None => {
                self.pipelines_data = PipelinesData::Errors(Error::msg("Project not selected"));
            }
        };
    }

    fn next(&mut self) {
        if let PipelinesData::Loaded(pipelines) = &self.pipelines_data {
            if self.active_operation_index < pipelines.len() - 1 {
                self.active_operation_index += 1;
            }
        }
    }

    fn previous(&mut self) {
        if let PipelinesData::Loaded(_) = &self.pipelines_data {
            if self.active_operation_index > 0 {
                self.active_operation_index -= 1;
            }
        }
    }

    fn show_details(&mut self, state: &mut State) {
        let projects = &self.config.core.gitlab_projects;
        state.active_gitlab_project = Some(projects[self.active_operation_index].clone());
    }
}

impl Component for PipelinesViewerComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action, state: &mut State) -> Result<Option<Action>> {
        match action {
            Action::Next => self.next(),
            Action::Previous => self.previous(),
            Action::Enter => self.show_details(state),
            Action::FocusUp => state.focused_component = 0, // change to header
            Action::FocusDown => state.focused_component = 3, // change to footer
            Action::FocusLeft => state.focused_component = 1, // change to project selector
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                self.load_pipelines_data(state);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &State) -> Result<()> {
        let area = prepare_layout(area, Element::Body(Body::RightColumn));
        let block = get_block(state, 2, Color::Green);
        match &self.pipelines_data {
            PipelinesData::Loading => {
                let loading_message = vec![Line::from(Span::styled(
                    "Loading...",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))];
                let paragraph = Paragraph::new(loading_message)
                    .block(block)
                    .alignment(Alignment::Center);
                frame.render_widget(paragraph, area);
            }
            PipelinesData::Loaded(pipelines) => {
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
                    let hightlight_style = if i == self.active_operation_index {
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

                let paginator = build_paginator(pipelines.len(), self.active_page + 1);
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
                .header(header_row)
                .flex(Flex::SpaceAround)
                .block(
                    block
                        .padding(Padding::uniform(1))
                        .title("Pipelines")
                        .title(
                            Line::styled(
                                format!("Filters: {}", &self.active_filters.join(", ")),
                                Style::default().add_modifier(Modifier::ITALIC),
                            )
                            .right_aligned(),
                        )
                        .title_bottom(
                            Line::from(format!(
                                "{} of {}",
                                self.active_operation_index + 1,
                                pipelines.len()
                            ))
                            .right_aligned(),
                        )
                        .title_bottom(Line::from(format!("Pages: {}", paginator)).left_aligned()),
                );

                frame.render_widget(table, area);

                if self.show_details_popup {
                    let pipeline_id = pipelines[self.active_operation_index].id;
                    // TODO: here we need to send a request to GitLab to fetch some pipeline
                    // details (or process what we already have, tbd)
                    let block = Block::bordered().title(format!("Details for: {}", pipeline_id));
                    let area = popup_area(area, 60, 20);
                    frame.render_widget(Clear, area); // this clears out the background
                    frame.render_widget(block, area);
                }
            }
            PipelinesData::Errors(error) => {
                let loading_message = vec![Line::from(Span::styled(
                    format!("ERROR: {}", error),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ))];
                let paragraph = Paragraph::new(loading_message)
                    .block(block)
                    .alignment(Alignment::Center);
                frame.render_widget(paragraph, area);
            }
        }
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

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
