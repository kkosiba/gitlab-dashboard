pub struct State {
    pub render_project_selector: bool,
    pub active_project: Option<String>,
    pub active_operation_index: usize,
    pub active_filters: Vec<String>,
}
