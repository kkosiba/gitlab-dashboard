use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType},
};

use crate::state::State;

pub enum Body {
    LeftColumn,
    RightColumn,
}

pub enum Element {
    Header,
    Body(Body),
    Footer,
}

pub fn prepare_layout(area: Rect, position: Element) -> Rect {
    // First, we do the vertical split into header, body and footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(2), Constraint::Fill(1), Constraint::Max(3)])
        .split(area);

    // Then, depending on the position, we return the chunk which should be used to render specific
    // component
    match position {
        Element::Header => chunks[0],
        Element::Footer => chunks[2],
        Element::Body(body_position) => {
            // Here we split the main body into two columns
            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
                .split(chunks[1]);

            match body_position {
                Body::LeftColumn => body_chunks[0],
                Body::RightColumn => body_chunks[1],
            }
        }
    }
}

pub fn get_block(state: &State, focused_component: usize, focused_border_color: Color) -> Block {
    Block::bordered()
        .border_type(if state.focused_component == focused_component {
            BorderType::Thick
        } else {
            BorderType::Plain
        })
        .border_style(if state.focused_component == focused_component {
            Style::default().fg(focused_border_color)
        } else {
            Style::default()
        })
}
