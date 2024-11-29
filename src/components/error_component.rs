use color_eyre::{eyre::Error, Result};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct ErrorComponent {
    command_tx: Option<UnboundedSender<Action>>,
    state: ErrorState,
}

#[derive(Default)]
pub struct ErrorState {
    pub error_details: Option<Error>,
}

impl ErrorComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for ErrorComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        let _ = config; // config is not used by ErrorComponent
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
        match &self.state.error_details {
            Some(error_details) => {
                let loading_message = vec![Line::from(Span::styled(
                    format!("ERROR: {}", error_details),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ))];
                let block = Paragraph::new(loading_message).alignment(Alignment::Center);
                frame.render_widget(block, area);
            }
            None => {}
        }
        Ok(())
    }
}
