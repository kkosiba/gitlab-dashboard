use color_eyre::Result;
use ratatui::prelude::*;

use crate::state::State;

use super::Component;

#[derive(Default)]
pub struct HeaderComponent {}

impl HeaderComponent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for HeaderComponent {
    fn height_constraint(&self) -> Constraint {
        Constraint::Max(1)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &State) -> Result<()> {
        // Split the area into two horizontal chunks
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Render the left-aligned text
        frame.render_widget(
            Line::from(vec![Span::styled(
                format!("[ GitLab Pipelines Viewer {} v0.0.1 ]", symbols::DOT), // TODO: read project version dynamically
                Style::default().fg(Color::LightBlue),
            )]),
            chunks[0],
        );

        // Render the right-aligned text
        frame.render_widget(
            Line::from(vec![Span::styled(
                &state.active_gitlab_project,
                Style::default().fg(Color::LightRed),
            )])
            .right_aligned(),
            chunks[1],
        );

        Ok(())
    }
}
