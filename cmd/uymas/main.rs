extern crate cli;

use cli::action::Action;
use cli::args::Args;
use cli::cmd::{ActionApp, Cmd};
use cli::VERSION;

struct Version {
    //arg: Option<Args>,
}

impl Action for Version {
    fn run(&self, _: &Args) {
        println!("v{}", VERSION)
    }
}

// 绑定信息
fn action_help(_: &Args) {
    println!("命令如下：");
    println!("test      参数解析测试");
    println!("version   版本号输出");
    println!();
    println!("全局选项：");
    println!(" --version,-v          输出版本号");
    println!(" --help,-h,-?          查看帮助信息");
    println!();
}

// 二进制文件
fn main() {
    let mut cmd = Cmd::from(vec![]);
    let version = ActionApp {
        command: String::from("version"),
        alias: vec![],
        action: Box::new(Version {}),
    };

    cmd.register_action(Box::new(version));

    // 命令不存在
    cmd.un_found(|args: &Args| println!("{} 命令未存在！", args.command));

    // test
    cmd.register("test", |args: &Args| {
        println!("command: {}", args.command);
        println!("sub_command: {}", args.sub_command);
        println!("option: {:?}", args.option);
        println!("data: {:?}", args.data);
        println!("raw: {:?}", args.raw);
    });

    // help
    cmd.register("help", action_help);

    // 默认方法
    cmd.empty(|args: &Args| {
        if args.contain_opts(vec!["version", "v"]) {
            let version = Version {};
            version.run(args);
            return;
        } else if args.contain_opts(vec!["help", "?", "h"]) {
            action_help(args);
            return;
        }
        println!("$ uymas [command] [option]");
        println!("uymas 命令行工具");
        println!("uymas_cli 目标是创建快速依赖最小的命令行库");
        println!("v{}", VERSION);
    });
    cmd.run();
}
