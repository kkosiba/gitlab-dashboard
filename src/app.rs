use crossterm::event::{self, Event, KeyCode};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::Frame;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};
use std::time::Duration;

use crate::config::Config;
use crate::gitlab::fetch_pipelines;
use crate::{
    state::{PipelinesData, State},
    ui::ui::{render_pipelines_view, render_project_selector},
};
use color_eyre::Result;

pub struct App {
    config: Config,
    state: State,
}

impl App {
    pub fn new(config: Config) -> Self {
        let state = State::default();
        Self { config, state }
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

    fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(terminal)
    }

    fn teardown_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
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

    pub async fn run(&mut self) -> Result<()> {
        let mut terminal = Self::setup_terminal()?;
        self.select_project();
        self.load_pipelines_data();

        loop {
            terminal.draw(|f| {
                self.draw(f);
            })?;

            if self.handle_event()? {
                break;
            }
        }
        Self::teardown_terminal(&mut terminal)?;
        Ok(())
    }

    fn handle_event(&mut self) -> Result<bool> {
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
