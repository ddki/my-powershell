use serde::Deserialize;

use super::job::Job;

#[derive(Debug, Deserialize)]
pub struct Workflow {
    name: Option<String>,
    version: Option<String>,
    author: Option<String>,
    description: Option<String>,
    repository: Option<String>,
    proxy: Option<String>,
    jobs: Option<Vec<Job>>
}