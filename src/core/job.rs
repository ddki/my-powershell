use std::{collections::HashMap, ops::Add, thread, time::Duration};

use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    sort: Option<u32>,
    must: Option<bool>,
    name: Option<String>,
    description: Option<String>,
    url: Option<String>,
    pre_command: Option<Vec<CommandInfo>>,
    post_command: Option<Vec<CommandInfo>>,
    installs: Option<Vec<CommandInfo>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandInfo {
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    args: Option<HashMap<String, String>>,
}

impl Job {
    pub fn execute(&self) {
        // debug log
        if cfg!(debug_assertions) {
            println!();
            crate::util::print_line_title_default(&format!(
                "🏗️   开始执行 Job {}-{}",
                self.sort.as_ref().unwrap(),
                self.name.as_ref().unwrap()
            ));
            println!("{}", style(self.to_toml()).blue().dim());
        }
        // 日志样式
        let spinner_style =
            ProgressStyle::with_template("{prefix:.bold.blue.dim} {spinner} {wide_msg}")
                .unwrap()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
        let mp = MultiProgress::new();
        // 执行前置命令
        match &self.pre_command {
            Some(commands) => {
                let pb = mp.add(ProgressBar::new(commands.len().try_into().unwrap()));
                pb.set_style(spinner_style.clone());
                pb.set_prefix("[pre_command]");
                for command in commands {
                    command.execute();
                    pb.set_message(command.full_command());
                    pb.inc(1);
                }
                pb.finish_with_message("✔️ pre_command finished！");
                let _ = mp.clear();
            }
            None => {}
        }

        // 执行安装命令
        match &self.installs {
            Some(installs) => {
                let pb = mp.add(ProgressBar::new(installs.len().try_into().unwrap()));
                pb.set_style(spinner_style.clone());
                pb.set_prefix("[install_command]");
                for install in installs {
                    install.execute();
                    pb.set_message(install.full_command());
                    pb.inc(1);
                }
                pb.finish_with_message("✔️ install_command finished！");
                let _ = mp.clear();
            }
            None => {}
        }

        // 执行后置命令
        match &self.post_command {
            Some(commands) => {
                let pb = mp.add(ProgressBar::new(commands.len().try_into().unwrap()));
                pb.set_style(spinner_style.clone());
                pb.set_prefix("[post_command]");
                for command in commands {
                    command.execute();
                    pb.set_message(command.full_command());
                    pb.inc(1);
                }
                pb.finish_with_message("✔️ post_command finished！");
                let _ = mp.clear();
            }
            None => {}
        }
    }

    pub fn get_sort(&self) -> u32 {
        self.sort.as_ref().unwrap().clone()
    }

    pub fn to_toml(&self) -> String {
        toml::to_string(self).unwrap_or_default()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn must(&self) -> bool {
        self.must.unwrap_or_default()
    }

    pub fn name(&self) -> String {
        self.name.as_ref().unwrap_or(&String::default()).clone()
    }

    pub fn description(&self) -> String {
        self.description.as_ref().unwrap_or(&String::default()).clone()
    }
}

impl CommandInfo {
    pub fn execute(&self) {
        let _commands = self.command.as_ref().unwrap_or(&String::default()).clone();
        thread::sleep(Duration::from_millis(2000));
    }

    pub fn full_command(&self) -> String {
        self.command.as_ref().unwrap_or(&String::default()).clone()
    }

    fn args_str(&self) -> String {
        return match &self.args {
            Some(args) => {
                let mut args_str = String::new();
                let start = args.get("start").unwrap_or(&String::default()).clone();
                for (key, value) in args.iter() {
                    if key == "start" {
                        continue;
                    }
                    // todo tui输入
                    args_str += &format!("{}{} ", start, value);
                }
                return args_str;
            }
            None => String::default(),
        };
    }

    pub fn to_toml(&self) -> String {
        toml::to_string(self).unwrap_or_default()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}
