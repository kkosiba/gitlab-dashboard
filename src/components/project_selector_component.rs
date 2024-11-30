use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{utils::centered_layout, Component};
use crate::{action::Action, config::Config};

#[derive(Default)]
struct ProjectSelectorState {
    active_operation_index: usize,
    active_project: Option<String>,
    should_render: bool,
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

    fn select_project(&mut self) {
        let projects = &self.config.core.gitlab_projects;
        // At this point we're guaranteed to have at least one GitLab project in the config file.
        if projects.len() > 1 {
            self.state.should_render = true;
        }
        self.state.active_project = Some(projects[0].to_string());
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
            Action::Render => self.select_project(),
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        if !self.state.should_render {
            Ok(())
        } else {
            let area = centered_layout(area);
            let projects = &self.config.core.gitlab_projects;
            let list_items: Vec<ListItem> =
                projects.iter().map(|i| ListItem::new(i.clone())).collect();

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
}
