// 命令行路由器
use std::env;
use std::vec::Vec;

// @TODO rust 生命周期/所有权太复杂；编写困难，与其他语言比较
// @TODO 2019年6月18日 星期二 无法编译通过
pub struct Router<'a> {
    args: Vec<&'a str>,
}

impl<'a> Router<'a> {
    // 默认参数为命令行输入的参数
    // 实例化
    pub fn new() -> Router<'a> {
        let mut new_args: Vec<&str> = vec![];
        /*for (i, arg) in env::args().enumerate() {
            // i += 1;
            // if i == 1 {
            //     continue;
            // }
            // new_args.push(arg.as_str())
            println!("{} -> {}", i, arg);
            //new_args.push(&arg.as_str());
        }*/
        let args: Vec<String> = env::args().collect();
        for arg in &args {
            new_args.push(&arg.as_str());
        }
        println!("{:?}", new_args);
        Router { args: vec![] }
    }

    // 内部值路由
    fn _router(&self) {}

    // 设置参数
    // 可用于内部程序调试
    pub fn args(&mut self, args: Vec<&'a str>) -> &Router {
        if args.len() > 0 {
            self.args = args;
        }
        self
    }

    // 路由注册
    pub fn register(&self, cmd: &str) -> &Router {
        // @TODO 注册处理
        println!("{}", cmd);
        return self;
    }

    // 路由器运行
    pub fn run(&self) {}
}
