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

pub type CmdCallOption = Option<Box<dyn FnMut(&Args)>>;
pub type CmdCallMap = HashMap<String, Box<dyn FnMut(&Args)>>;

/// 命令行工具，使用实现命令行注册
///
///
/// # Examples
/// 默认使用 `start` 指定命令行程序，
/// ```
/// use uymas_cli::args::Args;
/// use uymas_cli::cmd::Cmd;
/// let mut cmd = Cmd::new();
///
/// // 默认调用
/// cmd.empty(|_: &Args|{
///     println!("Hello World, Cli.");
/// });
///
/// // 注册方法
/// cmd.register("version", |_: &Args|{
///     println!("v1.0.0");
/// });
///
/// // 方法指定
/// cmd.start();
/// ```
///
/// 使用 run 执行命令行，支持输入参数
/// ```
/// use uymas_cli::cmd::{Cmd, CmdRunOs};
/// let mut cmd = Cmd::new();
/// cmd.run();
/// ```
/// 支持 `Vec<String>` 参数
/// ```
/// use uymas_cli::cmd::{Cmd, CmdRunString};
/// let mut cmd = Cmd::new();
/// cmd.run(vec!["git".to_string(), "status".to_string()]);
/// ```
/// 支持 &str 参数
/// ```
/// use uymas_cli::cmd::{Cmd, CmdRunStr};
/// let mut cmd = Cmd::new();
/// cmd.run("git status");
/// ```
/// 支持 Vec<&str> 参数
/// ```
/// use uymas_cli::cmd::{Cmd, CmdRunArgs};
/// let mut cmd = Cmd::new();
/// cmd.run(vec!["git", "status"]);
/// ```
pub struct Cmd {
    raw_args: Vec<String>,            //原始输入参数
    calls: CmdCallMap,                // （注册的）函数集合
    actions: Vec<ActionApp>,          // 方法集合
    action_default: CmdCallOption,    // 默认执行方法
    action_no_handler: CmdCallOption, // 不存在的处理方法时
    args: Option<Args>,
    cmd_alias: Option<HashMap<String, Vec<String>>>, // 命令别名类别，函数定义时
}

/// 将 `env::args` 转化为 `Vec<String>`
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
    /// 运行命令行应用
    fn run(&mut self, param: Vec<&str>);

    /// 多个命令注册命令
    fn register_multi<F>(&mut self, names: Vec<&str>, action: F) -> &mut Self
    where
        F: FnMut(&Args) + 'static;
}

/// 来源自定义 `str` 转化为 Args实例化的命令行
pub trait CmdRunStr {
    fn run(&mut self, param: &str);
}

/// 来源自定义 `String` 转化为 Args实例化的命令行
pub trait CmdRunString {
    /// 运行命令行应用
    fn run(&mut self, param: Vec<String>);

    /// 多个命令注册命令
    fn register_multi<F>(&mut self, names: Vec<String>, action: F) -> &mut Self
    where
        F: FnMut(&Args) + 'static;
}

// 为结构体添加方法
impl Cmd {
    /// 命令行方法调用
    pub fn new() -> Cmd {
        Cmd {
            ..Default::default()
        }
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

    /// 命令方法注册
    pub fn register<F>(&mut self, name: &str, action: F) -> &mut Cmd
    where
        F: FnMut(&Args) + 'static,
    {
        self.calls.insert(String::from(name), Box::new(action));
        self
    }

    /// 命令方法注册
    pub fn registers<F>(&mut self, names: Vec<&str>, action: F) -> &mut Self
    where
        F: FnMut(&Args) + 'static,
    {
        self.try_register_multi(names, action)
    }

    /// app 类型应用注册
    pub fn register_action(&mut self, app: ActionApp) -> &mut Cmd {
        self.actions.push(app);
        self
    }

    /// 命令行默认执行方法
    pub fn empty<F>(&mut self, action: F) -> &mut Cmd
    where
        F: FnMut(&Args) + 'static,
    {
        self.action_default = Some(Box::new(action));
        self
    }

    /// 处理应用不匹配时的调用
    pub fn un_found<F>(&mut self, action: F) -> &mut Cmd
    where
        F: FnMut(&Args) + 'static,
    {
        self.action_no_handler = Some(Box::new(action));
        self
    }

    /// 命令行默认调整
    pub fn start(&mut self) {
        self.parse_args();
        self.try_router();
    }

    // 尝试执行路由
    fn try_router(&mut self) {
        if self.args.is_none() {
            println!("因无Args参数，Cmd 运行失败");
            return;
        }
        let args = self.args.as_ref().unwrap();
        // 函数式定义参数
        for (v_key, v_fn) in &mut self.calls {
            if args.command == *v_key {
                v_fn(args);
                return;
            }
        }
        // 函数式定义参数（别名）
        if let Some(cmd_alias) = &self.cmd_alias {
            for (ca_key, ca_val) in cmd_alias {
                let mut is_match = args.command == *ca_key.as_str();
                if !is_match {
                    for cv in ca_val {
                        if args.command == *cv.as_str() {
                            is_match = true;
                            break;
                        }
                    }
                }
                if is_match {
                    if let Some(v_fn) = self.calls.get_mut(ca_key.as_str()) {
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
                    if *alias == args.command {
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
                return;
            }
        }

        // 默认参数
        if let Some(action_default) = &mut self.action_default {
            action_default(args);
        } else {
            println!("请您至少为 Cmd 应用注册默认方法");
        }
    }

    // 多命令注册
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
    /// use uymas_cli::cmd::{Cmd, CmdRunArgs};
    /// let mut cmd = Cmd::new();
    /// cmd.run();
    /// ```
    fn run(&mut self) {
        let args = Args::from_os();
        self.set_args(args).start();
    }
}

impl CmdRunArgs for Cmd {
    /// 来源与`Vec<String>`的参数
    /// ```
    /// use uymas_cli::cmd::{Cmd, CmdRunArgs};
    /// let mut cmd = Cmd::new();
    /// cmd.run(vec!["git", "log", "--stat"]);
    /// ```
    fn run(&mut self, param: Vec<&str>) {
        let args = Args::new(param);
        self.set_args(args).start();
    }

    /// 多命令注册
    /// ```
    /// use uymas_cli::args::Args;
    /// use uymas_cli::cmd::{Cmd, CmdRunArgs};
    ///
    /// let mut cmd = Cmd::new();
    /// cmd.register_multi(vec!["version", "v"], |_: &Args| println!("v1.0.0"));
    /// cmd.run(vec!["git", "log", "--stat"]);
    /// ```
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
    /// use uymas_cli::cmd::{Cmd, CmdRunArgs};
    /// let mut cmd = Cmd::new();
    /// cmd.run(vec!["git", "log", "--stat"]);
    /// ```
    fn run(&mut self, param: &str) {
        let args = Args::from_str(param);
        self.set_args(args).start();
    }
}

impl CmdRunString for Cmd {
    fn run(&mut self, param: Vec<String>) {
        let args = Args::new(param);
        self.set_args(args).start();
    }

    fn register_multi<F>(&mut self, names: Vec<String>, action: F) -> &mut Self
    where
        F: FnMut(&Args) + 'static,
    {
        // Vec<&str> -> Vec<String>
        let values: Vec<&str> = Vec::from_iter(names.iter().map(|x| x.as_str()));
        self.try_register_multi(values, action)
    }
}

/// 命令行默认执行方法
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
