#[derive(Default)]
pub struct State {
    pub active_gitlab_project: Option<String>,
    pub active_operation_index: usize,
    pub active_filter: String,
    pub input_mode: InputMode,
    // Focused components:
    // 0 - header
    // 1 - project selector
    // 2 - pipelines viewer
    // 3 - footer
    pub focused_component: usize,
}

#[derive(Default, PartialEq)]
pub enum InputMode {
    #[default]
    Normal,
    Insert,
    Command,
}
