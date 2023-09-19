use std::collections::HashMap;

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
    install: Option<Install>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Install {
    command: Option<Vec<String>>,
    args: Option<HashMap<String, String>>
}

impl Job {
    pub fn execute(&self) {
        println!("\n==================== 🏗️ 开始执行 Job {:?} - {:?} ====================", self.sort.as_ref().unwrap(), self.name.as_ref().unwrap());
        println!("{}", self.to_toml());
        for install in self.install.as_ref().iter() {
            install.execute();
        }
    }

    pub fn get_sort(&self) -> u32 { self.sort.as_ref().unwrap().clone() }

    pub fn to_toml(&self) -> String { toml::to_string(self).or(Err("")).unwrap() }
}

impl Install {
    pub fn execute(&self) {
        let commands = self.command.as_ref().unwrap().clone();
        for command in commands {
            println!("执行 Install [{:?}]", command);
        }
    }

    pub fn to_toml(&self) -> String { toml::to_string(self).or(Err("")).unwrap() }
}