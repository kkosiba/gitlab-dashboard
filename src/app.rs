use std::sync::{Arc, Mutex};

#[derive(PartialEq)]
pub enum Pane {
    Left,
    Right,
}

pub enum ApiStatus {
    Loading,
    Loaded(Vec<String>),
}

pub struct App {
    pub api_status_left: Arc<Mutex<ApiStatus>>,
    pub api_status_right: Arc<Mutex<ApiStatus>>,
    pub left_index: usize,
    pub right_index: usize,
    pub active_pane: Pane,
    pub active_tab: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            api_status_left: Arc::new(Mutex::new(ApiStatus::Loading)),
            api_status_right: Arc::new(Mutex::new(ApiStatus::Loading)),
            left_index: 0,
            right_index: 0,
            active_pane: Pane::Left,
            active_tab: 0,
        }
    }

    pub fn next(&mut self) {
        match self.active_pane {
            Pane::Left => {
                if let ApiStatus::Loaded(lines) = &*self.api_status_left.lock().unwrap() {
                    if self.left_index < lines.len() - 1 {
                        self.left_index += 1;
                    }
                }
            }
            Pane::Right => {
                if let ApiStatus::Loaded(lines) = &*self.api_status_right.lock().unwrap() {
                    if self.right_index < lines.len() - 1 {
                        self.right_index += 1;
                    }
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

    pub fn next_tab(&mut self) {
        self.active_tab = (self.active_tab + 1) % 2;
    }

    pub fn previous_tab(&mut self) {
        if self.active_tab == 0 {
            self.active_tab = 1;
        } else {
            self.active_tab -= 1;
        }
    }
}
