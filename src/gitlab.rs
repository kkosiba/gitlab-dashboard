use color_eyre::Result;
use std::env;
use strum;

use chrono::{DateTime, Utc};
use color_eyre::eyre::Error;
use gitlab::{
    api::{self, projects::pipelines::Pipelines, Pagination, Query},
    Gitlab,
};
use serde::Deserialize;

#[derive(Default)]
pub enum PipelinesData {
    #[default]
    Loading,
    Loaded(Vec<GitlabPipeline>),
    Errors(Error),
}

/// Pipeline status, see
/// https://docs.gitlab.com/ee/api/pipelines.html for reference.
#[derive(PartialEq, Deserialize, strum::Display)]
#[serde(rename_all = "snake_case")]
pub enum PipelineStatus {
    Created,
    WaitingForResource,
    Preparing,
    Pending,
    Running,
    Success,
    Failed,
    Canceled,
    Skipped,
    Manual,
    Scheduled,
}

/// GitLab pipeline sources, see
/// https://docs.gitlab.com/ee/ci/jobs/job_rules.html#ci_pipeline_source-predefined-variable
/// for reference.
#[derive(Deserialize, strum::Display)]
#[serde(rename_all = "snake_case")]
pub enum PipelineSource {
    Push,
    Web,
    Trigger,
    Schedule,
    Api,
    External,
    Pipeline,
    Chat,
    WebIde,
    MergeRequestEvent,
    ExternalPullRequestEvent,
    ParentPipeline,
    OnDemandDastScan,
    OnDemandDastValidation,
    SecurityOrchestrationPolicy,
}

#[derive(Deserialize)]
#[non_exhaustive]
pub struct GitlabPipeline {
    pub id: u32,
    pub status: PipelineStatus,
    pub source: PipelineSource,
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub web_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn fetch_pipelines(
    gitlab_url: String,
    gitlab_project: String,
    pagination_limit: usize,
) -> Result<Vec<GitlabPipeline>> {
    let token = env::var("GITLAB_PERSONAL_ACCESS_TOKEN")?;
    let client = Gitlab::new(gitlab_url, token)?;
    let endpoint = Pipelines::builder().project(gitlab_project).build()?;
    let pipelines: Vec<GitlabPipeline> =
        api::paged(endpoint, Pagination::Limit(pagination_limit)).query(&client)?;
    Ok(pipelines)
}
