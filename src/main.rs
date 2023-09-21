use core::workflow::Workflow;
use std::{fs::File, io::ErrorKind};

use clap::Parser;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

pub mod constant;
pub mod core;
pub mod util;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    workflow: Option<Workflow>,
}

impl Config {
    pub fn new() -> Config {
        Self { workflow: None }
    }
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
    #[arg(
        long,
        default_value = "my-powershell.json",
        help = "配置文件，支持toml、json。默认：my-powershell.toml"
    )]
    config: String,
}

fn main() {
    let args = Args::parse();
    let config_path = &args.config;
    // 加载配置文件
    let _ = match File::open(config_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                for config_file_name in ["my-powershell.toml", "my-powershell.json"] {
                    // 加载默认配置文件
                    let default_config_template = Asset::get(config_file_name).unwrap();
                    // 配置文件不存在，创建新的默认配置文件
                    std::fs::write(
                        config_file_name,
                        std::str::from_utf8(default_config_template.data.as_ref()).unwrap(),
                    )
                    .expect("创建默认配置文件出错！");
                }
                // 返回配置文件
                File::open(config_path).unwrap()
            }
            _ => {
                panic!("无法打开配置文件：{:?}", error);
            }
        },
    };

    let config_str = std::fs::read_to_string(config_path).expect("打开配置文件出错！");
    let mut config: Config = Config::new();
    if let Some(suffix) = config_path.split(".").last() {
        match suffix {
            "json" => config = serde_json::from_str(&config_str).unwrap(),
            "toml" => config = toml::from_str(&config_str).unwrap(),
            _ => panic!("不支持 {} 格式的配置文件", suffix),
        }
    }
    // debug 模式打印日志
    if cfg!(debug_assertions) {
        println!("config = {:#?}", config);
        println!("json = {}", serde_json::to_string(&config).unwrap());
        println!("toml = {}", toml::to_string_pretty(&config).unwrap())
    }
    let workflow = config.workflow.as_ref().unwrap();
    // 执行工作流
    workflow.execute();
}
