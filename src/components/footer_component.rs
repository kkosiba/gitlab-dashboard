use color_eyre::Result;
use ratatui::prelude::*;

use crate::state::State;

use super::{
    utils::{prepare_layout, Element},
    Component,
};

#[derive(Default)]
pub struct FooterComponent {}

impl FooterComponent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for FooterComponent {
    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect, _state: &State) -> Result<()> {
        let area = prepare_layout(area, Element::Footer);
        frame.render_widget(
            Line::from(vec![
                Span::styled("[ ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("h/l - left/right pane {} ", symbols::DOT),
                    Style::default().fg(Color::Blue),
                ),
                Span::styled(
                    format!("j/k - next/prev item {} ", symbols::DOT),
                    Style::default().fg(Color::Blue),
                ),
                Span::styled(
                    format!("p - focus project selector {} ", symbols::DOT),
                    Style::default().fg(Color::Blue),
                ),
                Span::styled("q - quit", Style::default().fg(Color::Red)),
                Span::styled(" ]", Style::default().fg(Color::White)),
            ]),
            area,
        );

        Ok(())
    }
}
