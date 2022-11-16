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
    cmd_alias: Option<HashMap<String, Vec<String>>>, //命令别名类别，函数定义时
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
    fn register_multi<F>(&mut self, names: Vec<&str>, action: F) -> &mut Self
    where
        F: FnMut(&Args) + 'static;
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
            cmd_alias: None,
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
    pub fn start(&mut self) {
        self.parse_args();
        self.try_router();
    }

    // 尝试路由
    fn try_router(&mut self) {
        if self.args.is_none() {
            println!("因无Args参数，Cmd 运行失败");
            return;
        }
        let args = self.args.as_ref().unwrap();
        // 函数式定义参数
        for (v_key, v_fn) in &mut self.calls {
            if args.command == String::from(v_key) {
                v_fn(args);
                return;
            }
        }
        // 函数式定义参数（别名）
        if let Some(cmd_alias) = &self.cmd_alias {
            for (ca_key, ca_val) in cmd_alias {
                let mut is_match = args.command == String::from(ca_key.as_str());
                if !is_match {
                    for cv in ca_val {
                        if args.command == String::from(cv.as_str()) {
                            is_match = true;
                            break;
                        }
                    }
                }
                if is_match {
                    if let Some(v_fn) = self.calls.get_mut(ca_key.as_str()) {
                        //(v_fn)(args);
                        v_fn(args);
                        return;
                    }
                }
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
        if let Some(action_no_hdl) = &mut self.action_no_handler {
            if !args.command.is_empty() {
                action_no_hdl(args);
            }
        }

        // 默认参数
        if let Some(action_default) = &mut self.action_default {
            action_default(args);
        } else {
            println!("请您至少为 Cmd 应用注册默认方法");
        }
    }

    // 多删除注册
    fn try_register_multi<F>(&mut self, names: Vec<&str>, action: F) -> &mut Self
    where
        F: FnMut(&Args) + 'static,
    {
        let v_len = names.len();
        if v_len < 1 {
            return self;
        }
        let key = names[0];
        self.register(key, action);
        let mut cmd_alias: HashMap<String, Vec<String>> = HashMap::new();
        if let Some(v_cmd_alias) = &self.cmd_alias {
            cmd_alias = v_cmd_alias.clone();
        }

        // Vec<&str> -> Vec<String>
        let mut values: Vec<String> = Vec::from_iter(names.iter().map(|x| String::from(*x)));
        if let Some(old_val) = cmd_alias.get(key) {
            for old in old_val {
                values.push(old.to_string())
            }
        }

        cmd_alias.insert(String::from(key), values);
        self.cmd_alias = Some(cmd_alias);
        self
    }
}

impl CmdRunOs for Cmd {
    /// 来源与系统的参数
    /// ```
    ///     use uymas_cli::cmd::{Cmd, CmdRunArgs};
    ///     let mut cmd = Cmd::new();
    ///     cmd.run();
    /// ```
    fn run(&mut self) {
        let args = Args::from_os();
        self.set_args(args).start();
    }
}

impl CmdRunArgs for Cmd {
    /// 来源与`Vec<String>`的参数
    /// ```
    ///     use uymas_cli::cmd::{Cmd, CmdRunArgs};
    ///     let mut cmd = Cmd::new();
    ///     cmd.run(vec!["git", "log", "--stat"]);
    /// ```
    fn run(&mut self, param: Vec<&str>) {
        let args = Args::new(param);
        self.set_args(args).start();
    }

    /// 多命令注册
    fn register_multi<F>(&mut self, names: Vec<&str>, action: F) -> &mut Self
    where
        F: FnMut(&Args) + 'static,
    {
        self.try_register_multi(names, action)
    }
}

impl CmdRunStr for Cmd {
    /// 来源于字符串的参数
    /// ```
    ///     use uymas_cli::cmd::{Cmd, CmdRunArgs};
    ///     let mut cmd = Cmd::new();
    ///     cmd.run(vec!["git", "log", "--stat"]);
    /// ```
    fn run(&mut self, param: &str) {
        let args = Args::from_str(param);
        self.set_args(args).start();
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
            cmd_alias: None,
        }
    }
}
