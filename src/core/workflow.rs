use std::collections::HashMap;

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
        println!("==================== 🏗️ 开始执行 Workflow {:?} ====================", self.name.as_ref().unwrap());
        println!("[{:?}] 描述: {:?}", self.name.as_ref().unwrap(), self.description.as_ref().unwrap());
        println!("[{:?}] 版本: {:?}\t 作者: {:?}", self.name.as_ref().unwrap(), self.version.as_ref().unwrap(), self.author.as_ref().unwrap());
        println!("[{:?}] 仓库地址： {:?}", self.name.as_ref().unwrap(), self.repository.as_ref().unwrap());
        println!("[{:?}] 代理： {:?}", self.name.as_ref().unwrap(), self.proxy.as_ref().unwrap());
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