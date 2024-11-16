use std::fmt;

#[derive(Debug)]
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

impl fmt::Display for PipelineStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Pipeline {
    pub id: u32,
    pub status: PipelineStatus,
    /*
    We could define 'source' using enum, using data available here:
    https://docs.gitlab.com/ee/ci/jobs/job_rules.html#ci_pipeline_source-predefined-variable
    but at this point, let's keep it simple.
    */
    pub source: String,
}
