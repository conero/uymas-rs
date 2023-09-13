use crate::app::App;
use crate::config::Config;
#[cfg(feature = "log")]
use crate::logger::Logger;
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
#[cfg(feature = "log")]
mod logger;
// 系统应用IP
mod app;

fn log_init() {
    // 日志初始化
    if cfg!(feature = "log") {
        #[cfg(feature = "log")]
        Logger::init().unwrap();

        #[cfg(feature = "log")]
        log::info!("正启动 --features log……");
    }
}

fn main() {
    log_init();
    let mut app = cmd::Cmd::new();

    app.register("config", App::config);
    app.register("help", App::help);

    #[cfg(feature = "log")]
    app.register("log", App::log);

    app.un_found(App::noexist);

    // 系统入口命令
    app.empty(|arg: &Args| {
        if arg.contain_opts(vec!["help", "h"]) {
            App::help(arg);
            return;
        }
        println!("这一个系统命令行脚手架项目");
        println!();
        println!("包含主要功能如下：");
        println!(" .    系统配置文件 config ");
        println!();
    });

    app.run();
}
