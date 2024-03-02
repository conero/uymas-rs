# uymas-rs

> 2018年11月12日 星期一
>
> Joshua  Conero



[![Crates.io](https://img.shields.io/crates/v/uymas_cli?style=flat-square)](https://crates.io/crates/uymas_cli)
[![Crates.io](https://img.shields.io/crates/d/uymas_cli?style=flat-square)](https://crates.io/crates/uymas_cli)



### 项目介绍

- **cli**  命令行解析，实现简单快速的二进制命令行程序开发。由于https://github.com/clap-rs/clap 强大二进制库生成的二进制过去庞大，因此在[uymas](https://github.com/conero/uymas) 几乎上开发 rust 版本程序。



更多详情请前往：

- [doc.rs](https://crates.io/crates/uymas_cli)
- [example](https://gitee.com/conero/uymas-rs/tree/example/)



rust 入门级个人库

```shell
# 代码缓存清除
$ cargo clean

# 项目代码格式化
$ cargo fmt

# 编译代码
cargo build -p uymas --release

# 生成本地文档，并
cargo doc -p uymas_cli --open
```





#### 安装

```toml
[dependencies]
# 安装开发分支代码
cli = { git = "https://github.com/conero/uymas-rs", branch = "try", package="uymas_cli" }

# 安装 crate.io 发行版
uymas_cli = "2.0.0"
```





**代码实例**

```rust
use cli::args::Args;
use cli::cmd::{Cmd, CmdRunOs};

// test action
fn action_test(arg: &Args) {
    println!("exec: {}", cli::args::get_exec_path());
    println!("basedir: {}", cli::args::get_exec_dir());
    println!("Args: {:?}", arg);
}

fn main() {
    let mut app = Cmd::new();

    app.empty(|arg| {
        if arg.contain_opts(vec!["version", "V"]) {
            println!("v0.0.1");
            return;
        }
        println!("hello world, uymas cli!")
    });

    app.register("test", action_test);

    app.run();
}
```

