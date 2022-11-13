// 注入式命令

use cli::args::Args;

pub struct ReplCmd {
    arg: Option<Args>,
}

impl ReplCmd {
    pub fn new() -> ReplCmd {
        ReplCmd { arg: None }
    }

    // 执行
    pub fn run(&mut self, arg: &Args) {
        self.arg = Some(arg.clone());
        println!("Repl 应用 :)- ");
        loop {
            let mut input_string = String::new();
            print!("$> ");
            match std::io::stdin().read_line(&mut input_string) {
                Ok(_) => {
                    if input_string == "exit" {
                        println!("Bye, Have a good day!");
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
