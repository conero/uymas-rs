use crate::config::Config;
use cli::args::Args;
use cli::cmd;
use cli::cmd::CmdRunOs;
use lazy_static::lazy_static;

// 配置缓存
// https://crates.io/crates/lazy_static
lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

/// 系统配置
mod config;
/// 日志支持
#[cfg(log)]
mod log;

fn log_init() {
    // 日志初始化
    if cfg!(feature = "log") {}
}

fn main() {
    log_init();
    let mut app = cmd::Cmd::new();

    // 系统如可命令
    app.empty(|_: &Args| {
        println!("这一个系统命令行脚手架项目");
        println!();
        println!("包含主要功能如下：");
        println!(" .    系统配置文件 config ");
        println!();
    });

    app.run();
}
