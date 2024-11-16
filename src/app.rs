use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::Alignment;
use ratatui::widgets::{Block, Borders, ListItem, Paragraph, Row, Table};
use ratatui::{backend::Backend, Terminal};
use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    Frame,
};
use std::os::unix::process;
use std::{error::Error, time::Duration};

use crate::config::Config;
use crate::gitlab::{Pipeline, PipelineStatus};
use crate::state::State;

enum PipelinesData {
    Loading, // TODO: Use this variant when API data is being fetched
    Loaded(Vec<Pipeline>),
}

pub struct App {
    config: Config,
    pipelines_data: PipelinesData,
    state: State,
}

impl App {
    pub fn new(config: Config) -> Self {
        let state = State {
            render_project_selector: false,
            active_project: None,
            active_operation_index: 0,
            active_filters: vec![String::from("ALL")],
        };
        Self {
            config,
            pipelines_data: PipelinesData::Loaded(vec![
                Pipeline {
                    id: 1,
                    status: PipelineStatus::Success,
                    source: String::from("push"),
                },
                Pipeline {
                    id: 2,
                    status: PipelineStatus::Failed,
                    source: String::from("merge_event"),
                },
                Pipeline {
                    id: 3,
                    status: PipelineStatus::Running,
                    source: String::from("scheduled"),
                },
            ]),
            state,
        }
    }

    fn next(&mut self) {
        match &self.pipelines_data {
            PipelinesData::Loading => {}
            PipelinesData::Loaded(pipelines) => {
                if self.state.active_operation_index < pipelines.len() - 1 {
                    self.state.active_operation_index += 1;
                }
            }
        }
    }

    fn previous(&mut self) {
        match &self.pipelines_data {
            PipelinesData::Loading => {}
            PipelinesData::Loaded(_) => {
                if self.state.active_operation_index > 0 {
                    self.state.active_operation_index -= 1;
                }
            }
        }
    }

    fn select_project(&mut self) {
        let projects = &self.config.core.gitlab_projects;
        // At this point we're guaranteed to have at least one GitLab project in the config file.
        if &projects.len() > &1 {
            self.state.render_project_selector = true;
        }
        self.state.active_project = Some(projects[0].to_string());
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        self.select_project();

        loop {
            terminal.draw(|f| {
                self.draw(f);
            })?;

            if self.handle_event()? {
                break;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self) -> Result<bool, Box<dyn Error>> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('j') | KeyCode::Down => self.next(),
                    KeyCode::Char('k') | KeyCode::Up => self.previous(),
                    _ => {}
                }
            }
        }
        Ok(false)
    }

    fn draw(&self, f: &mut Frame) {
        let area = f.area();

        if self.state.render_project_selector {
            // Render project selector
            unimplemented!("Implement me!")
        } else {
            // Render pipelines view
            match &self.pipelines_data {
                PipelinesData::Loading => {
                    let loading_message = vec![Line::from(Span::styled(
                        "Loading...",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ))];
                    let block = Paragraph::new(loading_message).alignment(Alignment::Center);
                    f.render_widget(block, area);
                }
                PipelinesData::Loaded(pipelines) => {
                    let rows = pipelines.iter().enumerate().map(|(i, pipeline)| {
                        let style = if i == self.state.active_operation_index {
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
                            ),
                    );

                    f.render_widget(table, area);
                }
            }
        }
    }
}
