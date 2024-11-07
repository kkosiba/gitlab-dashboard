pub struct App {
    pub lines: Vec<String>,
    pub selected_index: usize,
}

impl App {
    pub fn new(lines: Vec<String>) -> Self {
        App {
            lines,
            selected_index: 0,
        }
    }

    pub fn next(&mut self) {
        if self.selected_index < self.lines.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}
