# 资源

> 2018年11月12日 星期一



> 参考文档

[Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/)





当前库文档生成并打开或常用命令：

```shell
# 生成文档
cargo doc --open

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
```





### 概述



> **系统分支**

- master           主分支(未定分支)
- try                   开发分支（需保证可正常编译）
- try-choke       try 开发分支引起的错误（可能编译不过）



