use cli::cmd::{Cmd, CmdRunOs};

fn main() {
    let mut app = Cmd::new();
    app.empty(|_| {
        println!("Hello, world!");
    });
    app.run();
}
