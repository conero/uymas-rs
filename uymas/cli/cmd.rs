use crate::action::Action;
use crate::args::{Args, ArgsNew};
use std::collections::HashMap;
use std::env;

// 类型别名
// type ActionFn = fn(&Args);

pub struct ActionApp {
    pub command: String, // 命令
    pub alias: Vec<String>,
    pub action: Box<dyn Action>,
}

/// 命令行工具，使用实现命令行注册
pub struct Cmd {
    raw_args: Vec<String>,
    calls: HashMap<String, Box<dyn FnMut(&Args)>>, // 函数集合
    actions: Vec<ActionApp>,                       //方法集合
    action_default: Option<Box<dyn FnMut(&Args)>>, // 默认执行方法
    action_no_handler: Option<Box<dyn FnMut(&Args)>>, // 不存在的处理方法时
    args: Option<Args>,
}

/// 将 `env::args` 转化为 Vec<String>
pub fn get_os_args() -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut idx = 0;
    for arg in env::args() {
        if idx < 1 {
            idx += 1;
            continue;
        }
        args.push(arg);
    }
    args
}

/// 来自Os::Args实例化的命令行
pub trait CmdRunOs {
    fn run(&mut self);
}

/// 来源自定义Args实例化的命令行
pub trait CmdRunArgs {
    fn run(&mut self, param: Vec<&str>);
}

/// 来源自定义Args实例化的命令行
pub trait CmdRunStr {
    fn run(&mut self, param: &str);
}

// 为结构体添加方法
impl Cmd {
    /// 命令行方法调用
    pub fn new() -> Cmd {
        Cmd {
            ..Default::default()
        }
    }

    /// 通过参数初始化命令行程序
    /// # Examples
    /// ```
    ///     use uymas_cli::cmd::Cmd;
    ///     let app = Cmd::from(vec!["log", "--stat"]);
    /// ```
    pub fn from(param: Vec<&str>) -> Cmd {
        let mut args: Vec<String> = Vec::new();
        for arg in param {
            args.push(String::from(arg));
        }
        let mut app = Cmd {
            raw_args: args,
            calls: HashMap::new(),
            action_default: None,
            action_no_handler: None,
            args: None,
            actions: Vec::new(),
        };
        app.parse_args();
        return app;
    }

    /// 通过 `str` 初始化 Cmd
    pub fn from_str(param: &str) -> Cmd {
        let args = Args::from_str(param);
        let mut cmd = Cmd {
            ..Default::default()
        };
        cmd.args = Some(args);
        cmd
    }

    // 获取操作系统命令
    fn get_os_args(&mut self) {
        self.raw_args = get_os_args();
    }

    // 解析参数
    fn parse_args(&mut self) {
        if self.raw_args.is_empty() {
            self.get_os_args()
        }
        if self.args.is_none() {
            let args = Args::new(&self.raw_args);
            self.args = Some(args);
        }
    }

    // 设置参数
    fn set_args(&mut self, args: Args) -> &mut Cmd {
        self.args = Some(args);
        self
    }

    // 方法注册
    pub fn register<F>(&mut self, name: &str, action: F) -> &mut Cmd
    where
        F: FnMut(&Args) + 'static,
    {
        self.calls.insert(String::from(name), Box::new(action));
        self
    }

    pub fn register_action(&mut self, app: Box<ActionApp>) -> &mut Cmd {
        self.actions.push(*app);
        self
    }

    // 默认方法
    pub fn empty<F>(&mut self, action: F) -> &mut Cmd
    where
        F: FnMut(&Args) + 'static,
    {
        self.action_default = Some(Box::new(action));
        self
    }

    // 不存时处理
    pub fn un_found<F>(&mut self, action: F) -> &mut Cmd
    where
        F: FnMut(&Args) + 'static,
    {
        self.action_no_handler = Some(Box::new(action));
        self
    }

    // 命令行执行
    pub fn run(&mut self) {
        self.parse_args();
        //@todo 需要实现，直接调用 `try_router`
        //self.try_router();
    }

    // 尝试执行 router
    // @todo 只能执行一次
    //pub fn try_router(&self) {
    pub fn try_router(self) {
        if self.args.is_none() {
            println!("因无Args参数，Cmd 运行失败");
            return;
        }
        let args = self.args.as_ref().unwrap();
        // 函数式定义参数
        for (v_key, mut v_fn) in self.calls {
            if args.command == String::from(v_key) {
                v_fn(args);
                return;
            }
        }

        // 类名定义尝试
        for action in &self.actions {
            if action.command == args.command {
                action.action.as_ref().run(args);
                return;
            } else {
                for alias in &action.alias {
                    if String::from(alias) == args.command {
                        action.action.as_ref().run(args);
                        return;
                    }
                }
            }
        }

        // 命令不存在时
        if !self.action_no_handler.is_none() && !args.command.is_empty() {
            (self.action_no_handler.unwrap())(args);
            return;
        }

        // 默认参数
        if !self.action_default.is_none() {
            (self.action_default.unwrap())(args);
        } else {
            println!("请您至少为 Cmd 应用注册默认方法");
        }
    }
}

impl CmdRunOs for Cmd {
    /// 来源与系统的参数
    /// ```
    ///     use uymas_cli::cmd::{Cmd, CmdRunArgs};
    ///     let cmd = Cmd::new();
    /// ```
    fn run(&mut self) {
        let args = Args::from_os();
        self.set_args(args).run();
    }
}

impl CmdRunArgs for Cmd {
    fn run(&mut self, param: Vec<&str>) {
        let args = Args::new(param);
        self.set_args(args).run();
    }
}

impl CmdRunStr for Cmd {
    fn run(&mut self, param: &str) {
        let args = Args::from_str(param);
        self.set_args(args).run();
    }
}

// 默认类型
impl Default for Cmd {
    fn default() -> Self {
        Cmd {
            raw_args: vec![],
            calls: Default::default(),
            actions: vec![],
            action_default: None,
            action_no_handler: None,
            args: None,
        }
    }
}
