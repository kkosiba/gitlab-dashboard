use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{
    utils::{prepare_layout, Body, Element},
    Component,
};
use crate::{action::Action, config::Config, state::State};

#[derive(Default)]
pub struct ProjectSelectorComponent {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    active_operation_index: usize,
}

impl ProjectSelectorComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn next(&mut self) {
        let projects = &self.config.core.gitlab_projects;
        if self.active_operation_index < projects.len() - 1 {
            self.active_operation_index += 1;
        }
    }

    fn previous(&mut self) {
        if self.active_operation_index > 0 {
            self.active_operation_index -= 1;
        }
    }

    fn select_project(&mut self, state: &mut State) {
        let projects = &self.config.core.gitlab_projects;
        state.active_gitlab_project = Some(projects[self.active_operation_index].clone());
    }
}

impl Component for ProjectSelectorComponent {
    fn height_constraint(&self) -> Constraint {
        Constraint::Fill(3)
    }

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
            Action::Enter => self.select_project(state),
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, _state: &State) -> Result<()> {
        let area = prepare_layout(area, Element::Body(Body::LeftColumn));
        let projects = &self.config.core.gitlab_projects;
        let list_items: Vec<ListItem> = projects.iter().map(|i| ListItem::new(i.clone())).collect();

        let mut list_state = ListState::default();
        list_state.select(Some(self.active_operation_index));

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
