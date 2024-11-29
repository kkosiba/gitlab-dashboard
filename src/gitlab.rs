use color_eyre::Result;
use std::env;

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
#[derive(PartialEq, Deserialize)]
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

impl ToString for PipelineStatus {
    fn to_string(&self) -> String {
        match self {
            PipelineStatus::Created => String::from("created"),
            PipelineStatus::WaitingForResource => String::from("waiting_for_resource"),
            PipelineStatus::Preparing => String::from("preparing"),
            PipelineStatus::Pending => String::from("pending"),
            PipelineStatus::Running => String::from("running"),
            PipelineStatus::Success => String::from("success"),
            PipelineStatus::Failed => String::from("failed"),
            PipelineStatus::Canceled => String::from("canceled"),
            PipelineStatus::Skipped => String::from("skipped"),
            PipelineStatus::Manual => String::from("manual"),
            PipelineStatus::Scheduled => String::from("scheduled"),
        }
    }
}

/// GitLab pipeline sources, see
/// https://docs.gitlab.com/ee/ci/jobs/job_rules.html#ci_pipeline_source-predefined-variable
/// for reference.
#[derive(Deserialize)]
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

impl ToString for PipelineSource {
    fn to_string(&self) -> String {
        match self {
            PipelineSource::Push => String::from("push"),
            PipelineSource::Web => String::from("web"),
            PipelineSource::Trigger => String::from("trigger"),
            PipelineSource::Schedule => String::from("schedule"),
            PipelineSource::Api => String::from("api"),
            PipelineSource::External => String::from("external"),
            PipelineSource::Pipeline => String::from("pipeline"),
            PipelineSource::Chat => String::from("chat"),
            PipelineSource::WebIde => String::from("web_ide"),
            PipelineSource::MergeRequestEvent => String::from("merge_request_event"),
            PipelineSource::ExternalPullRequestEvent => String::from("external_pull_request_event"),
            PipelineSource::ParentPipeline => String::from("parent_pipeline"),
            PipelineSource::OnDemandDastScan => String::from("ondemand_dast_scan"),
            PipelineSource::OnDemandDastValidation => String::from("ondemand_dast_validation"),
            PipelineSource::SecurityOrchestrationPolicy => {
                String::from("security_orchestration_policy")
            }
        }
    }
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
