use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::Alignment;
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table};
use ratatui::{backend::Backend, Terminal};
use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    Frame,
};
use std::{error::Error, time::Duration};

use crate::config::Config;

enum PipelinesData {
    Loading, // TODO: Use this variant when API data is being fetched
    Loaded(Vec<String>),
}

pub struct App {
    config: Config,
    pipelines_data: PipelinesData,
    index: usize,
    active_filters: Vec<String>, // TODO: Expand this later
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            pipelines_data: PipelinesData::Loaded(vec![
                String::from("Pipeline 1"),
                String::from("Pipeline 2"),
                String::from("Pipeline 3"),
            ]),
            index: 0,
            active_filters: vec![String::from("ALL")],
        }
    }

    fn next(&mut self) {
        match &self.pipelines_data {
            PipelinesData::Loading => {}
            PipelinesData::Loaded(pipelines) => {
                if self.index < pipelines.len() - 1 {
                    self.index += 1;
                }
            }
        }
    }

    fn previous(&mut self) {
        match &self.pipelines_data {
            PipelinesData::Loading => {}
            PipelinesData::Loaded(_) => {
                if self.index > 0 {
                    self.index -= 1;
                }
            }
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
                    let style = if i == self.index {
                        Style::default().fg(Color::Black).bg(Color::White)
                    } else {
                        Style::default()
                    };
                    Row::new(vec![Span::raw(pipeline)]).style(style)
                });

                let table = Table::new(rows, [Constraint::Percentage(100)]).block(
                    Block::default()
                        .title("Pipelines")
                        .title(
                            Line::styled(
                                format!("Filters: {}", &self.active_filters.join(", ")),
                                Style::default().add_modifier(Modifier::ITALIC),
                            )
                            .right_aligned(),
                        )
                        .borders(Borders::ALL)
                        .title_bottom(
                            Line::from(format!("{} of {}", self.index + 1, pipelines.len()))
                                .right_aligned(),
                        ),
                );

                f.render_widget(table, area);
            }
        }
    }
}
