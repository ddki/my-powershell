use std::{
    collections::{hash_map::RandomState, HashMap},
    env,
    io::{self, BufRead},
    process::{Command, Stdio},
};

use console::style;
use dialoguer::{theme::ColorfulTheme, Input};
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
    full_command: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandArgInfo {
    need_input: Option<bool>,
    value: Option<String>,
    label: Option<String>,
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
                let mut success_flag = true;
                for command in commands {
                    if command.clone().execute().is_err() {
                        success_flag = false;
                    }
                    pb.set_message(command.clone().full_command_str());
                    pb.inc(1);
                }
                if success_flag {
                    pb.finish_with_message("✔️ pre_command finished！");
                } else {
                    pb.finish_with_message("❌ pre_command finished！");
                }
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
                let mut success_flag = true;
                for install in installs {
                    if install.clone().execute().is_err() {
                        success_flag = false;
                    }
                    pb.set_message(install.clone().full_command_str());
                    pb.inc(1);
                }
                if success_flag {
                    pb.finish_with_message("✔️ install_command finished！");
                } else {
                    pb.finish_with_message("❌ install_command finished！");
                }
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
                let mut success_flag = true;
                for command in commands {
                    if command.clone().execute().is_err() {
                        success_flag = false;
                    }
                    pb.set_message(command.clone().full_command_str());
                    pb.inc(1);
                }
                if success_flag {
                    pb.finish_with_message("✔️ post_command finished！");
                } else {
                    pb.finish_with_message("❌ post_command finished！");
                }

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
    pub fn execute(&mut self) -> Result<String, &'static str> {
        // 构建tui，输入参数值
        self.args_tui();
        // 获取参数
        let args_str = self.args_str();

        match self.clone().command {
            Some(command_str) => {
                if let Ok(current_dir) = env::current_dir() {
                    let current_dir_str = current_dir.to_string_lossy().into_owned();
                    // 设置全命令
                    let full_command = format!("{} {}", &command_str, &args_str);
                    if cfg!(debug_assertions) {
                        println!("full_command: {}", full_command);
                    }
                    self.set_full_command(Some(full_command));

                    // 创建执行命令
                    let mut command = Command::new(&command_str);
                    command.current_dir(&current_dir_str);
                    let args_flag = !args_str.is_empty();
                    if args_flag {
                        command.arg(&args_str);
                    }

                    // powershell 脚本
                    if command_str.contains(".ps1") {
                        command = Command::new("powershell");
                        command.current_dir(&current_dir_str);
                        command
                            .arg("-ExecutionPolicy")
                            .arg("Bypass")
                            .arg("-File")
                            .arg(&command_str);
                        if args_flag {
                            command.arg(&command_str).arg(&args_str).stdout(Stdio::piped());
                        }
                    }

                    let mut output =
                        command.stdout(Stdio::piped()).spawn().expect("执行命令发生错误！");

                    if let Some(stdout) = output.stdout.take() {
                        let reader = io::BufReader::new(stdout);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                println!("{}", line);
                            }
                        }
                    }

                    let status = output.wait().expect("failed to wait for child process");
                    if status.success() {
                        println!("😃 命令执行成功！");
                        Ok(String::from("😃 命令执行成功！"))
                    } else {
                        println!("😢 命令执行失败！");
                        Err("😢 命令执行失败！")
                    }
                } else {
                    println!("😢 获取当前程序目录失败！");
                    Err("😢 获取当前程序目录失败！")
                }
            }
            None => Ok(String::default()),
        }
    }

    // 构建tui，输入参数值
    pub fn args_tui(&mut self) {
        match &self.args {
            Some(args) => {
                let mut input_args = HashMap::new();
                for (key, value) in args {
                    let inpur_value: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt(format!("请输入{}：", &value))
                        .allow_empty(true)
                        .interact_text()
                        .unwrap();
                    input_args.insert(key.to_string(), inpur_value);
                }
                println!("{:#?} {:?}", self, input_args);
                if !input_args.is_empty() {
                    self.set_args(Some(input_args));
                }
            }
            None => {}
        }
    }

    fn args_str(&mut self) -> String {
        return match &self.args {
            Some(args) => {
                let mut args_str = String::new();
                let start = args.get("start").unwrap_or(&String::default()).clone();
                let joiner = args.get("joiner").unwrap_or(&String::from(" ")).clone();
                for (key, value) in args.iter() {
                    if key == "start" || key == "joiner" || key.is_empty() || value.is_empty() {
                        continue;
                    }
                    args_str += &format!("{}{}{}{} ", start, key, joiner, value);
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

    pub fn set_args(&mut self, args: Option<HashMap<String, String>>) {
        self.args = args;
    }

    pub fn args(&self) -> Option<&HashMap<String, String, RandomState>> {
        self.args.as_ref()
    }

    pub fn set_full_command(&mut self, full_command: Option<String>) {
        self.full_command = full_command;
    }

    pub fn full_command_str(&mut self) -> String {
        self.full_command.as_ref().unwrap_or(&String::default()).to_string()
    }
}
