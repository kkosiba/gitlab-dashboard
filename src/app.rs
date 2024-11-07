#[derive(PartialEq)]
pub enum Pane {
    Left,
    Right,
}

pub struct App {
    pub left_lines: Vec<String>,
    pub right_lines: Vec<String>,
    pub left_index: usize,
    pub right_index: usize,
    pub active_pane: Pane,
}

impl App {
    pub fn new(left_lines: Vec<String>, right_lines: Vec<String>) -> Self {
        App {
            left_lines,
            right_lines,
            left_index: 0,
            right_index: 0,
            active_pane: Pane::Left,
        }
    }

    pub fn next(&mut self) {
        match self.active_pane {
            Pane::Left => {
                if self.left_index < self.left_lines.len() - 1 {
                    self.left_index += 1;
                }
            }
            Pane::Right => {
                if self.right_index < self.right_lines.len() - 1 {
                    self.right_index += 1;
                }
            }
        }
    }

    pub fn previous(&mut self) {
        match self.active_pane {
            Pane::Left => {
                if self.left_index > 0 {
                    self.left_index -= 1;
                }
            }
            Pane::Right => {
                if self.right_index > 0 {
                    self.right_index -= 1;
                }
            }
        }
    }

    pub fn switch_to_left(&mut self) {
        self.active_pane = Pane::Left;
    }

    pub fn switch_to_right(&mut self) {
        self.active_pane = Pane::Right;
    }
}
