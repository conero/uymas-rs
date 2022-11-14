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
pub trait CmdFromOs {
    fn new() -> Cmd;
}

/// 来源自定义Args实例化的命令行
pub trait CmdFromArgs {
    fn new(param: Vec<&str>) -> Cmd;
}

// 为结构体添加方法
impl Cmd {
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
    pub fn run(mut self) {
        if self.args.is_none() {
            let args = Args::new(&self.raw_args);
            self.args = Some(args);
        }

        // 函数式定义参数
        for (v_key, mut v_fn) in self.calls {
            if self.args.as_ref().unwrap().command == String::from(v_key) {
                v_fn(self.args.as_ref().unwrap());
                return;
            }
        }

        // 类名定义尝试
        for action in &self.actions {
            if action.command == self.args.as_ref().unwrap().command {
                action.action.as_ref().run(self.args.as_ref().unwrap());
                return;
            } else {
                for alias in &action.alias {
                    if String::from(alias) == self.args.as_ref().unwrap().command {
                        action.action.as_ref().run(self.args.as_ref().unwrap());
                        return;
                    }
                }
            }
        }

        // 命令不存在时
        if !self.action_no_handler.is_none() && !self.args.as_ref().unwrap().command.is_empty() {
            (self.action_no_handler.unwrap())(self.args.as_ref().unwrap());
            return;
        }

        // 默认参数
        if !self.action_default.is_none() {
            (self.action_default.unwrap())(self.args.as_ref().unwrap());
        }
    }
}

impl CmdFromOs for Cmd {
    /// 来源与系统的参数
    /// ```
    ///     use uymas_cli::cmd::{Cmd, CmdFromArgs};
    ///     let cmd = Cmd::new();
    /// ```
    fn new() -> Cmd {
        let mut cmd = Cmd::from(vec![]);
        cmd.get_os_args();
        cmd
    }
}

impl CmdFromArgs for Cmd {
    fn new(param: Vec<&str>) -> Cmd {
        Cmd::from(param)
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
