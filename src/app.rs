use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;
use ratatui::{backend::Backend, Terminal};
use std::{error::Error, time::Duration};

use crate::config::Config;
use crate::state::{PipelinesData, State};
use crate::ui::ui::{render_pipelines_view, render_project_selector};

pub struct App {
    config: Config,
    state: State,
}

impl App {
    pub fn new(config: Config) -> Self {
        let state = State::default();
        Self { config, state }
    }

    fn next(&mut self) {
        match &self.state.pipelines_data {
            PipelinesData::Loading | PipelinesData::Errors(_) => {}
            PipelinesData::Loaded(pipelines) => {
                if self.state.active_operation_index < pipelines.len() - 1 {
                    self.state.active_operation_index += 1;
                }
            }
        }
    }

    fn previous(&mut self) {
        match &self.state.pipelines_data {
            PipelinesData::Loading | PipelinesData::Errors(_) => {}
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
        if projects.len() > 1 {
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
        // TODO: This method has grown a bit already, consider refactoring it and maybe even moving
        // event handling to a separate module
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if self.state.render_project_selector {
                    match key.code {
                        KeyCode::Char('q') => return Ok(true),
                        KeyCode::Char('j') | KeyCode::Down => {
                            let projects = &self.config.core.gitlab_projects;
                            if self.state.active_operation_index < projects.len() - 1 {
                                self.state.active_operation_index += 1;
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if self.state.active_operation_index > 0 {
                                self.state.active_operation_index -= 1;
                            }
                        }
                        KeyCode::Enter => {
                            let projects = &self.config.core.gitlab_projects;
                            self.state.active_project =
                                Some(projects[self.state.active_operation_index].clone());
                            self.state.render_project_selector = false;
                            // Reset index for pipelines view
                            self.state.active_operation_index = 0;
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => return Ok(true),
                        KeyCode::Char('j') | KeyCode::Down => self.next(),
                        KeyCode::Char('k') | KeyCode::Up => self.previous(),
                        _ => {}
                    }
                }
            }
        }
        Ok(false)
    }

    fn draw(&self, f: &mut Frame) {
        if self.state.render_project_selector {
            let projects = &self.config.core.gitlab_projects;
            render_project_selector(f, &self.state, projects);
        } else {
            render_pipelines_view(f, &self.state);
        }
    }
}
