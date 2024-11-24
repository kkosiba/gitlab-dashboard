use std::{env, error::Error};

use gitlab::{
    api::{
        self,
        projects::pipelines::{PipelineSource, PipelineStatus, Pipelines},
        Pagination, Query,
    },
    Gitlab,
};

#[derive(Debug)]
pub struct GitlabPipeline {
    pub id: u32,
    pub status: PipelineStatus,
    pub source: PipelineSource,
}

pub fn fetch_pipelines(
    gitlab_url: String,
    gitlab_project: String,
    pagination_limit: usize,
) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
    let token = env::var("GITLAB_PERSONAL_ACCESS_TOKEN")?;
    let client = Gitlab::new(gitlab_url, token)?;

    let endpoint = Pipelines::builder().project(gitlab_project).build()?;

    // TODO: deserialize pipeline information into proper struct
    let pipelines: Vec<serde_json::Value> =
        api::paged(endpoint, Pagination::Limit(pagination_limit)).query(&client)?;

    Ok(pipelines)
}
