use std::time::Instant;

use console::style;
use indicatif::{HumanDuration, ProgressBar};
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
    jobs: Option<Vec<Job>>,
}

impl Workflow {
    pub fn execute(&self) {
        let started = Instant::now();
        crate::util::print_line_title_default(&format!(
            "🏗️   开始执行 Workflow {}",
            self.name.as_ref().unwrap()
        ));
        println!("📄 描述: {}", style(self.description.as_ref().unwrap()).blue());
        println!("🔖 版本: {}", style(self.version.as_ref().unwrap()).blue());
        println!("😄 作者: {}", style(self.author.as_ref().unwrap()).blue());
        println!("🔗 仓库地址： {}", style(self.repository.as_ref().unwrap()).blue());
        println!("⚡ 代理： {}", style(self.proxy.as_ref().unwrap()).blue());
        // todo 设置代理
        let mut jobs = Vec::<Job>::new();
        for job in self.jobs.as_ref().unwrap().iter() {
            jobs.push(job.clone());
        }
        // 排序
        jobs.sort_by_key(|job| {
            let temp_job = job.clone();
            temp_job.get_sort()
        });
        let job_pb = ProgressBar::new(jobs.len().try_into().unwrap());
        // 执行
        for (index, job) in jobs.iter().enumerate() {
            println!(
                "{} {} ({})",
                style(format!("[Job]-[{}/{}]", index + 1, jobs.len())).bold().red().dim(),
                job.name(),
                job.description()
            );
            job.execute();
            job_pb.inc(1);
        }
        // job_pb.finish_and_clear();
        println!(
            "✨ {} 耗时：{}",
            style("工作流执行完成！").bold().green().dim(),
            HumanDuration(started.elapsed())
        );
    }

    pub fn to_toml(&self) -> String {
        toml::to_string(self).unwrap_or_default()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}
