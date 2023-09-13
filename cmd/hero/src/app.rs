use crate::CONFIG;
use cli::args::Args;

pub struct App {}

impl App {
    // 配置文件获取
    pub fn config(_: &Args) {
        let decode = toml::to_string(&CONFIG.clone());
        if decode.is_err() {
            println!("config 序列号字符串内容错误。\n  {}", decode.err().unwrap());
            return;
        }

        println!("-------------- config --------------");
        println!();
        println!("{}", decode.unwrap());
    }

    // 帮助提示
    pub fn help(arg: &Args) {
        let command = arg.sub_command.clone();
        if !command.is_empty() {
            println!("{} 命令不存在", command);
            return;
        }
        println!("config       配置信息查看");
    }

    #[cfg(feature = "log")]
    pub fn log(arg: &Args) {
        log::info!("日志测试服务");
        log::info!(" subcommand        {}", arg.sub_command);
    }

    // 命令不存在
    pub fn noexist(arg: &Args) {
        println!("{} 命令不存在！", arg.command);
    }
}
