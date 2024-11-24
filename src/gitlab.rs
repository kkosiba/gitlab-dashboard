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


}
