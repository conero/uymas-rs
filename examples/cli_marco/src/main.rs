use cli::cmd::{Cmd, CmdRunOs};
use cli::derive::{cli_command, CliApp};
use cli::CliApp;

#[derive(CliApp)]
struct MarcoApp {}

impl MarcoApp {
    #[cli_command(Help, "help")]
    pub fn help(&self) {
        println!("help   命令");
        // 测试代码
    }
}

fn main() {
    let app = MarcoApp {};
    app.run();
    // 指定 help
    // app.help();
    //dummy();
    MarcoApp::dummy();
}
