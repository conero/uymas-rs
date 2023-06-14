# 系统发布日志

> 2019年6月18日 星期二



### todo

- [x] uymas 编译模式下，二进制还是比较大。与 [hi-lang](https://github.com/conero/lang/tree/hi-lang/hi-rust/learning22) 中相差较大 （`#221030`）



### v2.1.1/dev

> 代码根据 `cargo cliappy` 提示进行整改优化。

- **cli**
  - pref!(Cmd.register_action): 传递参数更改为非“Box”处理



### v2.1.0/2023-06-08

- **cli**
  - feat(Cmd): 新增方法 `from_str` 用于实现 `&str` 初始化 `Cmd `
  - feat(Cmd): 实现 `Default` trait
  - feat(Cmd): 尝试调整方法结构，是`Cmd::new` 后可多次 `Cmd.run`。
  - feat(CmdRunString): 新增trait 使用实现 `Vec<String>` 方法的请求
  - feat(args::Args): 新增基于范类的 `get_option` 方法获取值
  - feat(args::Args): 新增基于范类的 `get_option_vec` 方法获取参数列表
  - break(Cmd): 删除方法 `from` 和 `from_str` 方法。改变方法基础为 `new -> run`
  - feat: 新增方法 `project_path、get_exec_dir、get_exec_name、root_path_split、` 用于实现二进制所在目录的文件路径
- **uymas**
  - pref: `repl`添加 help, 以及默认命令



### v2.0.2/2022-11-14

- **cli**
  
  - feat(Args): 新增方法 `get_option_x` 获取整形、bool、浮点型等
  
  - pref(Args)!: 使用 `cmd` 函数注册支持 `move`
  
  - pref(Args): 将测试脚本移到项目的“tests”目录下
  
  - pref(Args): 方法 `get_value_x` 调整根据 `get_option_x` 获取，并可忽略异常 
  
  - pref(Args): 参数解析支持`--key=value`
  
  - pref(cli): Args 参数解析使用方法减少代码冗余
  
  - pref(cli): Args 尝试继承 Copy 
- **uymas**
  - feat: uymas 新增 repl 命令 
  



### v2.0.1/2022-11-02

- **system**
  - fix: 修复 `Cargo.` 配置无效的问题，在*workspace*环境下需配置在root下的meta配置中
- **cli**
  - feat: `Args.contain_opts`  新增方式用于检测 option是否存在
  - feat: `Args` 现在获取参数，如`stirng` ,  `i32`, `bool` 获取不懂类型的参数值
  - feat: `ArgsNew<T>` 新增trait 实现`Args`初始化不同参数重载
  - feat: `Args.from_str` 方法新增实现，字符串到 Args 类型的转换
  - feat: `Cmd` 通过 trait 实现 `Cmd::new` 方法重载，及支持空参数和非空参数实例化类型
  - fix:  修复 command、data 解析规则错误
- **uymas**
  - feat: 新增 `--version` , `--help` 等选项命令
  - pref: 帮助命令描述完善





### v2.0.0/2022-10-30

因此系统已使用 v1版本号，遂本地版本命名为 v2。

- feat: 使用 rust v2021版本，使用新学习到的语法优化代码库。重新激活项目代码
- feat: 项目结构化调整，即 cmd 表示二进制程序，uymas为项目代码
- **cli**
  - feat: 使用函数闭包，实现对函数式命令定义的支持
  - feat: 初步实现app cli trait



### 0.1.0-alpha/20190618

- (+) `util`  工具
  - (+) *实现 `util.Decimal.to_n` 用于十与任何进制之间的转换*
- (+) `bin` 新增命令行程序
  - (实现) *搭建基本的程序架构*

