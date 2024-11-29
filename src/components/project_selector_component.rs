use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
struct ProjectSelectorState {
    active_operation_index: usize,
    projects: Vec<String>,
}

#[derive(Default)]
pub struct ProjectSelectorComponent {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    state: ProjectSelectorState,
}

impl ProjectSelectorComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for ProjectSelectorComponent {
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
        let area = centered_layout(area);
        let list_items: Vec<ListItem> = self
            .state
            .projects
            .iter()
            .map(|i| ListItem::new(i.clone()))
            .collect();

        let mut list_state = ListState::default();
        list_state.select(Some(self.state.active_operation_index));

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

        frame.render_stateful_widget(list, area, &mut list_state);
        Ok(())
    }
}

fn centered_layout(area: Rect) -> Rect {
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
