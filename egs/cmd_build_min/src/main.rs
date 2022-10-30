use cli::args::Args;
use cli::cmd::Cmd;
use cli::VERSION;

fn main() {
    let mut cmd = Cmd::from(vec![]);
    cmd.empty(|_: &Args| {
        println!("uymas: v{}", VERSION);
        println!("Hello, world!");
    });
    cmd.run();
}
