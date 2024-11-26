use std::error::Error;

use crate::gitlab::GitlabPipeline;

pub enum PipelinesData {
    Loading,
    Loaded(Vec<GitlabPipeline>),
    Errors(Box<dyn Error>),
}

pub struct State {
    pub render_project_selector: bool,
    pub active_project: Option<String>,
    pub active_operation_index: usize,
    pub active_filters: Vec<String>,
    pub active_page: usize,
    pub pipelines_data: PipelinesData,
}

impl Default for State {
    fn default() -> Self {
        Self {
            render_project_selector: false,
            active_project: None,
            active_operation_index: 0,
            active_filters: vec![],
            active_page: 1,
            pipelines_data: PipelinesData::Loading,
        }
    }
}
