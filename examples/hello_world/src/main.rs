use cli::cmd::{Cmd, CmdRunOs};

fn main() {
    let mut app = Cmd::new();

    // 默认引用
    app.empty(|args| {
        if args.contain_opts(vec!["version", "v"]) {
            println!("v0.1.0-20240324");
            return;
        }
        println!("Hello world, Uymas Cli lib.")
    });

    // 命令不存在
    app.un_found(|args| {
        println!("Error: {} 命令不存在！", args.command);
    });

    // 注册命令
    app.register("conero", |_| {
        println!("Hello, conero");
    });

    // 注册多命令
    app.registers(vec!["test", "t"], |_| {
        println!("注册命令集别名");
        println!();
        println!("Test, try do test ya!");
    });

    // 运行命令行
    app.run();
}
