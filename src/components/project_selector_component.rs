use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{
    utils::{get_block, prepare_layout, Body, Element},
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

    fn next(&mut self, state: &State) {
        if state.focused_component == 1 {
            let projects = &self.config.core.gitlab_projects;

            if self.active_operation_index < projects.len() - 1 {
                self.active_operation_index += 1;
            }
        }
    }

    fn previous(&mut self, state: &State) {
        if state.focused_component == 1 {
            if self.active_operation_index > 0 {
                self.active_operation_index -= 1;
            }
        }
    }

    fn select_project(&mut self, state: &mut State) {
        let projects = &self.config.core.gitlab_projects;
        state.active_gitlab_project = Some(projects[self.active_operation_index].clone());
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

    fn update(&mut self, action: Action, state: &mut State) -> Result<Option<Action>> {
        match action {
            Action::Next => self.next(state),
            Action::Previous => self.previous(state),
            Action::Enter => self.select_project(state),
            Action::FocusUp => state.focused_component = 0, // change to header
            Action::FocusDown => state.focused_component = 3, // change to footer
            Action::FocusRight => state.focused_component = 2, // change to pipelines viewer
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &State) -> Result<()> {
        let area = prepare_layout(area, Element::Body(Body::LeftColumn));
        let block = get_block(state, 1, Color::LightMagenta);
        let projects = &self.config.core.gitlab_projects;
        let list_items: Vec<ListItem> = projects.iter().map(|i| ListItem::new(i.clone())).collect();

        let mut list_state = ListState::default();
        list_state.select(Some(self.active_operation_index));

        let list = List::new(list_items)
            .block(
                block
                    .title("Project Selector")
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
