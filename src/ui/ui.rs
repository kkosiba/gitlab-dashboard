use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn centered_layout(area: Rect) -> Rect {
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
