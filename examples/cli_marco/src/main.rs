use cli::args::Args;
use cli::cmd::{Cmd, CmdRunOs};
use cli::derive::{cli_command, CliApp};
use cli::CliApp;

#[derive(CliApp)]
struct MarcoApp {}

impl MarcoApp {
    // 测试程序
    #[cli_command(Help, "help", "h")]
    fn help() {
        println!("help   命令");
        // 测试代码
    }

    // 空方法
    #[cli_command(Empty)]
    fn empty() {
        println!("默认代码")
    }
}

fn main() {
    let app = MarcoApp {};
    app.run();
    // 指定 help
    // app.help();
    //dummy();
    app.help();
    app.empty();
}
