extern crate cli;

use cli::action::Action;
use cli::args::Args;
use cli::cmd::{ActionApp, Cmd, CmdRunArgs, CmdRunOs};
use cli::{args, RELEASE, VERSION};
use std::time::Instant;

// 文件引入
include!(concat!(env!("OUT_DIR"), "/uymas-version.rs"));

// 注入式类型
mod repl_cmd;

struct Version {
    //arg: Option<Args>,
}

impl Action for Version {
    fn run(&self, args: &Args) {
        if args.contain_opts(vec!["verbose", "V"]) {
            println!(
                "v{}/{} ({}) {}",
                VERSION,
                RELEASE,
                current_git_hash(),
                build_date()
            );
            return;
        }
        println!("v{}", VERSION)
    }
}

// 绑定信息
fn action_help(_: &Args) {
    println!("命令如下：");
    println!("test,t    参数解析测试");
    println!("version   版本号输出，--verbose,-V 详细信息输出");
    println!("repl      交互式命令行测试");
    println!();
    println!("全局选项：");
    println!(" --version,-v          输出版本号");
    println!(" --help,-h,-?          查看帮助信息");
    println!();
}

fn action_test(arg: &Args) {
    println!("command: {}", arg.command);
    println!("sub_command: {}", arg.sub_command);
    println!("option: {:?}", arg.option);
    println!("data: {:?}", arg.data);
    println!("raw: {:?}", arg.raw);
    println!("cwd: {:?}", args::get_current_dir("").unwrap());
    if let Some(opt_test) = arg.get_option_string(vec!["var", "v"]) {
        println!(
            "test Option: {} => {:?}",
            opt_test.clone(),
            arg.get_value_string(vec![opt_test.as_str()])
        );
    }
    println!();
}

// 二进制文件
fn main() {
    let now = Instant::now();
    let mut cmd = Cmd::new();
    let version = ActionApp {
        command: "version".to_string(),
        alias: vec![],
        action: Box::new(Version {}),
    };

    cmd.register_action(version);

    // 命令不存在
    cmd.un_found(|args: &Args| println!("{} 命令未存在！", args.command));

    // test
    // move test
    cmd.register_multi(vec!["test", "t"], move |arg: &Args| {
        action_test(arg);
        println!();
        println!("用时：{} 毫秒(ms).", now.elapsed().as_micros());
        println!("应用所在目录：{}", args::get_exec_dir());
        println!();
        println!("输入命令 “--var,-v $name”   可用于读取 $name 的option参数");
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
        } else if args.contain_opts(vec!["test"]) {
            action_test(args);
            return;
        }
        println!("$ {} [command] [option]", args::get_exec_name());
        println!();
        println!(" uymas 命令行工具");
        println!(" uymas_cli 目标是创建快速依赖最小的命令行库");
        println!();
        println!(" v{}/{}", VERSION, RELEASE);
    });

    // 注入式代码处理
    let mut repl = repl_cmd::ReplCmd::new();
    cmd.register("repl", move |args: &Args| {
        repl.run(args);
    });

    // 一下两种方式皆可行
    CmdRunOs::run(&mut cmd);
    //cmd.start()
}
