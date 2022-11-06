use crate::cmd::get_os_args;
use std::collections::HashMap;
use std::fmt::Display;

// 系统参数
pub struct Args {
    // 命令行
    pub command: String,
    // 二级命令
    pub sub_command: String,
    // 参数选项
    pub option: Vec<String>,
    // 请求参数
    pub data: HashMap<String, Vec<String>>,
    pub raw: Vec<String>,
}

impl Args {
    /// 获取命令行参数
    pub fn get_data(&self, key: &str) -> Option<&Vec<String>> {
        if let Some(value) = self.get_value(vec![key]) {
            return Some(value);
        }
        None
    }

    /// 获取输入的值，支持多参数
    pub fn get_value(&self, keys: Vec<&str>) -> Option<&Vec<String>> {
        if self.data.is_empty() {
            return None;
        }

        // 获取参数
        for key in keys {
            if self.data.contains_key(key) {
                return self.data.get(key);
            }
        }
        None
    }

    // 获取字符串参数
    pub fn get_value_string(&self, keys: Vec<&str>) -> String {
        let mut value = String::new();
        if let Some(v) = self.get_value(keys) {
            value = String::from(v.join(" ").trim());
        }
        value
    }

    /// 获取 `Option<String>` 类型
    #[deprecated(since = "2.0.1", note = "use the `get_option_string` instead")]
    pub fn get_value_string_option(&self, keys: Vec<&str>) -> Option<String> {
        self.get_option_string(keys)
    }

    /// 获取 i32 数据类型
    pub fn get_value_i32(&self, keys: Vec<&str>) -> i32 {
        if let Some(v) = self.get_option_isize(keys) {
            return v as i32;
        }
        0
    }

    /// 获取 bool 类型数据
    pub fn get_value_bool(&self, keys: Vec<&str>) -> bool {
        return if let Some(v) = self.get_option_bool(keys) {
            v
        } else {
            false
        };
    }

    /// 获取 bool 类型数据
    pub fn get_value_f64(&self, keys: Vec<&str>) -> f64 {
        return if let Some(v) = self.get_option_f64(keys) {
            v
        } else {
            0.0
        };
    }

    /// 查看是否存在 options
    pub fn contain_opts(&self, keys: Vec<&str>) -> bool {
        for key in keys {
            if self.option.contains(&String::from(key)) {
                return true;
            }
        }
        false
    }

    fn _parse_option() {}

    /// 实例化函数，解析桉树为命令行。`Cmd.data` 解析参照 url.Values 解析规则，即默认为 Vec<String>
    /// ```
    ///  // 解析 "rustc --version" 命令
    /// use uymas_cli::cmd::{Cmd, CmdFromArgs};
    /// let cmd = Cmd::new(vec!["rustc", "--version"]);
    /// ```
    /// @todo 将 option 处理中重复的代码提取为方法执行以复用代码
    pub fn from_args(args: &Vec<String>) -> Args {
        // 字段信息
        let mut command = String::new(); // 命令
        let mut sub_command = String::new(); // 子命令
        let mut option = Vec::new();
        let mut data: HashMap<String, Vec<String>> = HashMap::new();

        // 辅助变量
        let mut count = 0;
        let mut last_opt = String::new(); // 最新的opt缓存
        let mut last_value: Vec<String> = Vec::new(); // 最新的value缓存

        for arg in args {
            let arg = &String::from(arg.trim());
            if arg.is_empty() || arg.len() < 1 {
                continue;
            }
            let mut long_opt: i32 = -1;
            let mut shot_opt: i32 = -1;
            let mut equal_opt: usize = 0; // 是否含等于
            if let Some(vi) = arg.find("--") {
                long_opt = vi as i32;
            };
            if let Some(vi) = arg.find("-") {
                shot_opt = vi as i32;
            };
            if let Some(vi) = arg.find("=") {
                equal_opt = vi;
            }
            let is_not_opt = shot_opt != 0;
            if count == 0 && is_not_opt {
                // 命令解析
                command = String::from(arg);
            } else if count == 1 && sub_command.len() == 0 && is_not_opt {
                // 子命令
                sub_command = String::from(arg);
            } else {
                if long_opt == 0 {
                    let (_, opt) = arg.split_at(2);
                    if !last_value.is_empty() && !last_opt.is_empty() {
                        if !data.contains_key(last_opt.as_str()) {
                            data.insert(last_opt.clone(), last_value);
                        } else {
                            let mut exits_value = data.get(last_opt.as_str()).unwrap().to_vec();
                            for lv in last_value {
                                exits_value.push(lv);
                            }
                            data.insert(last_opt.clone(), exits_value.to_vec());
                        }
                        last_value = Vec::new();
                    }
                    last_opt = String::from(opt);
                    if equal_opt > 0 {
                        // 处理含等于符号的选项
                        let (eq_key, eq_value) = last_opt.split_at(equal_opt - 2);
                        last_value.push(String::from(&eq_value[1..]));
                        last_opt = String::from(eq_key);
                    }
                    option.push(String::from(&last_opt));
                } else if shot_opt == 0 {
                    let (_, opt) = arg.split_at(1);
                    if !last_value.is_empty() && !last_opt.is_empty() {
                        if !data.contains_key(last_opt.as_str()) {
                            data.insert(last_opt.clone(), last_value);
                        } else {
                            let mut exits_value = data.get(last_opt.as_str()).unwrap().to_vec();
                            for lv in last_value {
                                exits_value.push(lv);
                            }
                            data.insert(last_opt.clone(), exits_value.to_vec());
                        }
                        last_value = Vec::new();
                    }
                    for by in opt.chars() {
                        last_opt = String::from(by);
                        option.push(String::from(&last_opt));
                    }
                    if equal_opt > 0 {
                        // 处理含等于符号的选项
                        let (eq_key, eq_value) = last_opt.split_at(equal_opt);
                        last_value.push(String::from(eq_value));
                        last_opt = String::from(eq_key);
                    }
                } else {
                    last_value.push(String::from(arg));
                }
            }
            count += 1;
        }

        // 临界值（最后出现的）
        if !last_value.is_empty() && !last_opt.is_empty() {
            if !data.contains_key(last_opt.as_str()) {
                data.insert(last_opt.clone(), last_value);
            }
        }

        Args {
            command,
            sub_command,
            option,
            data,
            raw: args.to_vec(),
        }
    }

    /// 根据 os::args 获取参数
    pub fn from_os() -> Args {
        Args::from_args(&get_os_args())
    }

    /// 根据字符串转 Args 参数
    pub fn from_str(param: &str) -> Args {
        let queue = param.split(' ').collect();
        let args: Vec<String> = into_string_vec(queue);
        Args::from_args(&args)
    }

    /// 获取
    pub fn get_option_string(&self, keys: Vec<&str>) -> Option<String> {
        if let Some(v) = self.get_value(keys) {
            let value = String::from(v.join(" ").trim());
            return Some(value);
        }
        None
    }

    /// 获取 i32 参数
    pub fn get_option_isize(&self, keys: Vec<&str>) -> Option<isize> {
        if let Some(v) = self.get_option_string(keys) {
            match v.parse::<isize>() {
                Ok(value) => return Some(value),
                Err(_) => (),
            };
        }
        None
    }

    /// 获取 bool 类型
    pub fn get_option_bool(&self, keys: Vec<&str>) -> Option<bool> {
        if let Some(v) = self.get_option_string(keys) {
            match v.parse::<bool>() {
                Ok(value) => return Some(value),
                Err(_) => (),
            };
            // 存在时默认为存在
            return Some(true);
        }
        None
    }

    /// 获取 f64 类型
    pub fn get_option_f64(&self, keys: Vec<&str>) -> Option<f64> {
        if let Some(v) = self.get_option_string(keys) {
            match v.parse::<f64>() {
                Ok(value) => return Some(value),
                Err(_) => (),
            };
        }
        None
    }
}

pub trait ArgsFromOs {
    fn new() -> Args;
}

impl ArgsFromOs for Args {
    fn new() -> Args {
        Args::from_os()
    }
}

pub trait ArgsNew<T> {
    fn new(param: T) -> Self;
}

/// Args 使用 `Vec<String>` 初始化
/// ```
/// use uymas_cli::args::{Args, ArgsNew};
/// let cmd = Args::new(&vec![String::from("git"), String::from("remote"), String::from("-v")]);
/// ```
impl ArgsNew<&Vec<String>> for Args {
    fn new(param: &Vec<String>) -> Self {
        Args::from_args(&param)
    }
}

/// Args 使用 `Vec<String>` 初始化
/// ```
/// use uymas_cli::args::{Args, ArgsNew};
/// let cmd = Args::new(vec![String::from("git"), String::from("remote"), String::from("-v")]);
/// ```
/// Args 使用 `Vec<String>` 初始化
/// ```
/// use uymas_cli::args::{Args, ArgsNew};
/// let cmd = Args::new(vec!["git", "remote", "-v"]);
/// ```
impl<T> ArgsNew<Vec<T>> for Args
where
    T: Display,
{
    fn new(param: Vec<T>) -> Self {
        let args: Vec<String> = into_string_vec(param);
        Args::from_args(&args)
    }
}

fn into_string_vec<T>(args: Vec<T>) -> Vec<String>
where
    T: Display,
{
    let mut value: Vec<String> = Vec::new();
    for arg in args {
        value.push(format!("{}", arg));
    }
    value
}
