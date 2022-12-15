use cli::cmd::{Cmd, CmdRunOs};
use cli::derive::CliApp;
use cli::CliApp;

#[derive(CliApp)]
struct MarcoApp {}

fn main() {
    let mut app = Cmd::new();
    app.empty(|_| {
        MarcoApp::run();
        println!("Hello, world!");
    });
    app.run();
}
