use core::workflow::Workflow;

use serde::Deserialize;


pub mod core;

#[derive(Debug, Deserialize)]
struct Config {
    workflow: Option<Workflow>
}

fn main() {
    let config_str = std::fs::read_to_string("E:/workspace/github-my/my-powershell/resources/my-powershell-startup.toml").expect("read config failure");
    let config: Config = toml::from_str(&config_str).unwrap();
    println!("config = {:#?}", config);
}
