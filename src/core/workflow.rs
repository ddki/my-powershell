use std::collections::HashMap;

use console::style;
use serde::{Deserialize, Serialize};

use super::job::Job;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workflow {
    name: Option<String>,
    version: Option<String>,
    author: Option<String>,
    description: Option<String>,
    repository: Option<String>,
    proxy: Option<String>,
    jobs: Option<HashMap<String, Job>>
}

impl Workflow {
    pub fn execute(&self) {
        crate::util::print_line_title_default(&format!("🏗️   开始执行 Workflow {}", self.name.as_ref().unwrap()));
        println!("📄 描述: {}", style(self.description.as_ref().unwrap()).blue());
        println!("🔖 版本: {}", style(self.version.as_ref().unwrap()).blue());
        println!("😄 作者: {}", style(self.author.as_ref().unwrap()).blue());
        println!("🔗 仓库地址： {}", style(self.repository.as_ref().unwrap()).blue());
        println!("⚡ 代理： {}", style(self.proxy.as_ref().unwrap()).blue());
        // todo 设置代理
        let mut jobs = Vec::<Job>::new();
        for (_, job) in self.jobs.as_ref().unwrap().iter() {
            jobs.push(job.clone());
        }
        // 排序
        jobs.sort_by_key(|job| {
            let temp_job = job.clone();
            temp_job.get_sort()
        });
        // 执行
        for job in jobs {
            job.execute();
        }
    }

    pub fn to_toml(&self) -> String { toml::to_string(self).or(Err("")).unwrap() }
}