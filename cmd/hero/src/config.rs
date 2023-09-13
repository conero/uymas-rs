use cli::args::project_path;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CfgLog {
    #[serde(default = "def_str_empty")]
    pub level: String,
    #[serde(default = "def_str_empty")]
    pub file: String,
}

// 配合下默认为空
fn def_str_empty() -> String {
    String::new()
}

/// 系统配置文件
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub log: CfgLog,
}

impl Config {
    pub fn new() -> Self {
        match read_file() {
            Ok(cfg) => cfg,
            _ => Default::default(),
        }
    }
}

/// 通过配置文件读取配置
pub fn read_file() -> Result<Config, String> {
    let read_rslt = File::open(project_path("config.toml"));
    if let Err(er) = read_rslt {
        return Err(format!("读取文件错误，\n  reference -> {}", er));
    }

    let mut buf = String::new();
    if let Err(msg) = read_rslt.unwrap().read_to_string(&mut buf) {
        return Err(format!("读取文件错误，\n  reference -> {}", msg));
    }
    let yaml_rslt = toml::from_str(buf.as_str());
    if let Err(er) = yaml_rslt {
        return Err(format!("读取解析错误，\n  reference -> {}", er));
    }

    return Ok(yaml_rslt.unwrap());
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log: CfgLog {
                level: "trace".to_string(),
                file: String::new(),
            },
        }
    }
}
