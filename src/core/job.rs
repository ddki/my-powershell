use std::collections::HashMap;

use console::style;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    sort: Option<u32>,
    must: Option<bool>,
    name: Option<String>,
    description: Option<String>,
    url: Option<String>,
    pre_command: Option<Vec<String>>,
    post_command: Option<Vec<String>>,
    installs: Option<Vec<Install>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Install {
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    args: Option<HashMap<String, String>>
}

impl Job {
    pub fn execute(&self) {
        println!();
        crate::util::print_line_title_default(&format!("🏗️   开始执行 Job {}-{}", self.sort.as_ref().unwrap(), self.name.as_ref().unwrap()));
        if cfg!(debug_assertions) {
            println!("{}", style(self.to_toml()).blue().dim());
        }
        for install in self.installs.as_ref().unwrap().iter() {
            install.execute();
        }
    }

    pub fn get_sort(&self) -> u32 { self.sort.as_ref().unwrap().clone() }

    pub fn to_toml(&self) -> String { toml::to_string(self).or(Err("")).unwrap() }
}

impl Install {
    pub fn execute(&self) {
        let _commands = self.command.as_ref().unwrap().clone();
        
    }

    pub fn to_toml(&self) -> String { toml::to_string(self).or(Err("")).unwrap() }
}