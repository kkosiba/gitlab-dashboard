#[derive(Default)]
pub struct State {
    pub active_gitlab_project: String,
    pub active_operation_index: usize,
    pub active_filter: String,
    pub input_mode: InputMode,
}

#[derive(Default, PartialEq)]
pub enum InputMode {
    #[default]
    Normal,
    Insert,
    Command,
}
