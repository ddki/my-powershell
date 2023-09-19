use core::workflow::Workflow;
use std::{fs::File, io::ErrorKind};

use serde::Deserialize;
use clap::Parser;
use rust_embed::RustEmbed;

pub mod core;

#[derive(Debug, Deserialize)]
struct Config {
    workflow: Option<Workflow>
}

#[derive(RustEmbed)]
#[folder = "resources"]
struct Asset;

// 命令参数
#[derive(Parser, Debug)]
#[command(name = "my-powershell")]
#[command(author = "ddki")]
#[command(version = "0.0.1")]
#[command(about = "My PowerShell", long_about = None)]
pub struct Args {
    #[arg(long, default_value = "my-powershell.toml", help = "配置文件。默认：my-powershell.toml")]
    config: String,
}

fn main() {
    let args = Args::parse();
    println!("args: {:?}", args);
    let config_path = &args.config;
    // 加载配置文件
    let _ = match File::open(config_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // 加载默认配置文件
                let default_config_template = Asset::get("my-powershell.toml").unwrap();
                // 配置文件不存在，创建一个新的默认配置文件
                std::fs::write(config_path, std::str::from_utf8(default_config_template.data.as_ref()).unwrap()).expect("创建默认配置文件出错！");
                // 这里可以做一些处理或返回一个默认的文件对象
                File::open(config_path).unwrap()
            },
            _ => {
                panic!("无法打开配置文件：{:?}", error);
            }
        }
    };

    let config_str = std::fs::read_to_string(config_path).expect("打开配置文件出错！");
    let config: Config = toml::from_str(&config_str).unwrap();
    // println!("config = {:#?}", config);
    let workflow = config.workflow.as_ref().unwrap();
    // 执行工作流
    workflow.execute();
}
