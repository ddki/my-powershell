use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Job {
    sort: Option<u32>,
    must: Option<bool>,
    name: Option<String>,
    description: Option<String>,
    url: Option<String>,
    pre_command: Option<Vec<String>>,
    post_command: Option<Vec<String>>,
    install: Option<Install>
}

#[derive(Debug, Deserialize)]
pub struct Install {
    command: Option<Vec<String>>,
    args: Option<HashMap<String, String>>
}
