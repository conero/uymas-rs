// 命令行路由器
use crate::std_path;
use std::env;
use std::vec::Vec;

// @TODO rust 生命周期/所有权太复杂；编写困难，与其他语言比较
// @TODO 2019年6月18日 星期二 无法编译通过
pub struct Router<'a> {
    args: Vec<&'a str>,
    _base_dir: &'a str, //当前项目路径
}

impl<'a> Router<'a> {
    // 默认参数为命令行输入的参数
    // 实例化
    pub fn new() -> Router<'a> {
        let mut router = Router {
            args: vec![],
            _base_dir: &"",
        };
        let mut new_args: Vec<&str> = vec![];
        let args: Vec<String> = env::args().collect();
        let mut i: i32 = -1;
        for arg in &args {
            i += 1;
            if i == 0 {
                //let mut tmp_arg: &str = &mut arg.as_str();
                let tmp_arg = std_path(&arg.as_str());
                //tmp_arg = std_path(&tmp_arg).as_str();
                //router._base_dir = *tmp_arg;
                println!("tmp_arg -> {}", tmp_arg);
                // @todo cannot return value referencing local variable `tmp_arg`
                // router._base_dir = &tmp_arg.as_str();
                continue;
            }
            new_args.push(&arg.as_str());
        }
        router._router(new_args);
        return router;
    }

    // 带参数的实例化
    pub fn new_args(args: Vec<&str>) -> Router<'a> {
        let router = Router {
            args: vec![],
            _base_dir: &"",
        };
        router._router(args);
        return router;
    }

    // 内部值路由
    fn _router(&self, args: Vec<&str>) {
        for arg in args {
            println!("_router -> {}", arg)
        }
    }

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
