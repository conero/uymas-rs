use cli::cmd::{Cmd, CmdRunOs};
use cli::derive::CliApp;
use cli::CliApp;

#[derive(CliApp)]
struct MarcoApp {}

fn main() {
    let app = MarcoApp {};
    app.run();
}
