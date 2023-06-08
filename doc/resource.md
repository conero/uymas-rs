# 资源

> 2018年11月12日 星期一



> 参考文档

[Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/)





当前库文档生成并打开或常用命令：

```shell
# 生成文档
cargo doc --open
# 生成全部文档
cargo doc --all-features --all

# 格式化
cargo fmt
```



编译二进制项目

```shell
# 编译
cargo build --bin uymas --manifest-path .\cmd\uymas\Cargo.toml

# 编译和运行
cargo run --bin uymas --manifest-path .\cmd\uymas\Cargo.toml

# 发布
cargo build --bin uymas --manifest-path .\cmd\uymas\Cargo.toml --release

# 发布 uymas_cli 库到 crates-io
cargo publish -p uymas_cli --registry crates-io
```





### 概述



> **系统分支**

- master           主分支(未定分支)
- try                   开发分支（需保证可正常编译）
- try-choke       try 开发分支引起的错误（可能编译不过）



### Learning

#### proc-macro

过程宏只能单独导出含宏的代码，不可出现其他的。

```
error: `proc-macro` crate types currently cannot export any items other than functions tagged with `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]`
```







### 附录

#### 参考

- [Rust中的闭包与关键字move](https://zhuanlan.zhihu.com/p/341815515)
