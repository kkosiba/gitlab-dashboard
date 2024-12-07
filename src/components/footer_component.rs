use color_eyre::Result;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Paragraph},
};

use crate::{action::Action, state::State};

use super::{
    utils::{get_block, prepare_layout, Element},
    Component,
};

#[derive(Default)]
pub struct FooterComponent {}

impl FooterComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for FooterComponent {
    fn update(&mut self, action: Action, state: &mut State) -> Result<Option<Action>> {
        match action {
            Action::FocusUp => state.focused_component = 2, // change to pipelines viewer
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &State) -> Result<()> {
        let area = prepare_layout(area, Element::Footer);
        let footer = Line::from_iter(
            vec![
                "Keybindigs: ",
                "j/k - next/prev item | ",
                "ENTER - select item | ",
                "SHIFT+h/j/k/l - change focus | ",
                "q - quit",
            ]
            .into_iter(),
        );
        let block = get_block(state, 3, Color::LightBlue);
        let paragraph = Paragraph::new(footer).block(block);
        frame.render_widget(paragraph, area);
        Ok(())
    }
}
