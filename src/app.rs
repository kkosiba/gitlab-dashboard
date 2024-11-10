use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::Backend, Terminal};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};
use std::{error::Error, time::Duration};

use crate::config::Config;

#[derive(PartialEq)]
enum Pane {
    Left,
    Right,
}

enum ApiStatus {
    Loading,
    Loaded(Vec<String>),
}

pub struct App {
    config: Config,
    api_status_left: ApiStatus,
    api_status_right: ApiStatus,
    left_index: usize,
    right_index: usize,
    active_pane: Pane,
    active_tab: usize,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_status_left: ApiStatus::Loaded(vec![
                String::from("Pipelines"),
                String::from("Schedules"),
            ]),
            api_status_right: ApiStatus::Loaded(vec![
                String::from("Pipeline 1"),
                String::from("Pipeline 2"),
                String::from("Pipeline 3"),
            ]),
            left_index: 0,
            right_index: 0,
            active_pane: Pane::Left,
            active_tab: 0,
        }
    }

    fn next(&mut self) {
        match self.active_pane {
            Pane::Left => {
                if let ApiStatus::Loaded(lines) = &self.api_status_left {
                    if self.left_index < lines.len() - 1 {
                        self.left_index += 1;
                    }
                }
            }
            Pane::Right => {
                if let ApiStatus::Loaded(lines) = &self.api_status_right {
                    if self.right_index < lines.len() - 1 {
                        self.right_index += 1;
                    }
                }
            }
        }
    }

    fn previous(&mut self) {
        match self.active_pane {
            Pane::Left => {
                if self.left_index > 0 {
                    self.left_index -= 1;
                }
            }
            Pane::Right => {
                if self.right_index > 0 {
                    self.right_index -= 1;
                }
            }
        }
    }

    fn switch_to_left(&mut self) {
        self.active_pane = Pane::Left;
    }

    fn switch_to_right(&mut self) {
        self.active_pane = Pane::Right;
    }

    fn get_projects(&self) -> &Vec<String> {
        &self.config.core.gitlab_projects
    }

    fn next_tab(&mut self) {
        self.active_tab = (self.active_tab + 1) % self.get_projects().len();
    }

    fn previous_tab(&mut self) {
        if self.active_tab == 0 {
            self.active_tab = 1;
        } else {
            self.active_tab -= 1;
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
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
                    KeyCode::Char('j') => self.next(),
                    KeyCode::Char('k') => self.previous(),
                    KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.switch_to_left()
                    }
                    KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.switch_to_right()
                    }
                    KeyCode::Tab => self.next_tab(),
                    KeyCode::BackTab => self.previous_tab(), // BackTab = Shift + Tab
                    _ => {}
                }
            }
        }
        Ok(false)
    }

    fn draw(&self, f: &mut Frame) {
        let size = f.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);
        self.render_tabs(f, chunks[0]);
        self.render_main_content(f, chunks[1]);
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect) {
        let tab_spans: Vec<Span> = self.get_projects().iter().map(Span::raw).collect();
        let tabs = Tabs::new(tab_spans)
            .select(self.active_tab)
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

    fn render_main_content(&self, f: &mut Frame, area: Rect) {
        let pane_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(area);
        self.render_pane(f, Pane::Left, pane_chunks[0], "CI/CD", false);
        self.render_pane(f, Pane::Right, pane_chunks[1], "Details", true);
    }

    fn render_pane(&self, f: &mut Frame, pane: Pane, area: Rect, title: &str, with_sub_tabs: bool) {
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
            let sub_tab_labels = ["Passed", "Failed", "Cancelled", "Skipped"];
            let sub_tab_spans: Vec<Span> = sub_tab_labels
                .iter()
                .map(|&label| Span::raw(label))
                .collect();
            let sub_tabs = Tabs::new(sub_tab_spans)
                .block(
                    Block::default()
                        .title("Filter by status")
                        .borders(Borders::ALL),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );

            f.render_widget(sub_tabs, sub_tabs_area);
        }

        let (data, index) = match pane {
            Pane::Left => (&self.api_status_left, self.left_index),
            Pane::Right => (&self.api_status_right, self.right_index),
        };

        let styled_lines: Vec<Line> = match data {
            ApiStatus::Loading => vec![Line::from(Span::raw("Loading..."))],
            ApiStatus::Loaded(content) => content
                .iter()
                .enumerate()
                .map(|(i, line)| {
                    let style = if i == index && self.active_pane == pane {
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
}
