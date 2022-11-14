// 注入式命令

use cli::args::Args;
use std::io::Write;

pub struct ReplCmd {
    arg: Option<Args>,
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
            print!("$> ");
            std::io::stdout().flush().expect("Fail");
            let mut input_string = String::new();
            match std::io::stdin().read_line(&mut input_string) {
                Ok(_) => {
                    input_string = input_string.trim().to_string();
                    if input_string == "exit" {
                        println!("Bye, Have a good day! Ha.");
                        break;
                    }
                    println!("{}", input_string);
                }
                Err(er) => {
                    println!("读取输入参数错误，{}", er);
                    break;
                }
            }
        }
    }
}
