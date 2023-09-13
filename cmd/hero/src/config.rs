use serde::{Deserialize, Serialize};

/// 系统配置文件
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Default::default()
    }
}
