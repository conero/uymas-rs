// 注入式命令

use cli::args::Args;
use cli::cmd::Cmd;
use std::io::Write;

pub struct ReplCmd {
    arg: Option<Args>,
}

struct App {}

impl App {
    fn help(_: &Args) {
        println!("exit           退出repl对话框");
        println!("q              退出repl对话框");
    }

    // 首页
    fn index(args: &Args) {
        let width = 18;
        println!("{:width$}{}", "Command:", args.command);
        println!("{:width$}{}", "SubCommand:", args.sub_command);
        println!("{:width$}{:?}", "Option:", args.option);
        println!("{:width$}{:?}", "Data:", args.data);
        println!("{:width$}{:?}", "Raw:", args.raw);
    }
}

impl ReplCmd {
    pub fn new() -> ReplCmd {
        ReplCmd { arg: None }
    }

    // 执行
    //参考: [Rust print消息不换行的方法](https://blog.csdn.net/hustlei/article/details/102511654)
    pub fn run(&mut self, arg: &Args) {
        self.arg = Some(arg.clone());
        println!("Repl 应用 :)- ");
        loop {
            print!("\n$> ");
            std::io::stdout().flush().expect("Fail");
            let mut input_string = String::new();
            match std::io::stdin().read_line(&mut input_string) {
                Ok(_) => {
                    input_string = input_string.trim().to_string();
                    if input_string == "exit" || input_string == "q" {
                        println!("Bye, Have a good day! Ha.");
                        break;
                    }
                    let mut cmd = Cmd::from_str(input_string.as_str());
                    cmd.empty(App::index);
                    cmd.register("help", App::help);
                    cmd.run();
                }
                Err(er) => {
                    println!("读取输入参数错误，{}", er);
                    break;
                }
            }
        }
    }
}
