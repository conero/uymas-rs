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
                let value = self.data.get(key);
                if value != None {
                    let vs = value.unwrap();
                    return Some(vs);
                }
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
    pub fn get_value_string_option(&self, keys: Vec<&str>) -> Option<String> {
        if let Some(v) = self.get_value(keys) {
            let value = String::from(v.join(" ").trim());
            return Some(value);
        }
        None
    }

    /// 获取 i32 数据类型
    pub fn get_value_i32(&self, keys: Vec<&str>) -> i32 {
        if let Some(v) = self.get_value_string_option(keys) {
            return v.parse::<i32>().unwrap();
        }
        0
    }

    pub fn get_value_bool(&self, keys: Vec<&str>) -> bool {
        return if let Some(v) = self.get_value_string_option(keys) {
            v.parse::<bool>().unwrap()
        } else {
            true
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

    /// 实例化函数，解析桉树为命令行。`Cmd.data` 解析参照 url.Values 解析规则，即默认为 Vec<String>
    /// ```
    ///  // 解析 "rustc --version" 命令
    /// use uymas_cli::cmd::{Cmd, CmdFromArgs};
    /// let cmd = Cmd::new(vec!["rustc", "--version"]);
    /// ```
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

        // 使用闭包函数
        // let update_data_value = |v_opt: &String| {
        //     if !v_opt.is_empty(){
        //         if !data.contains_key(v_opt.as_str()){
        //             data.insert(v_opt, last_value);
        //         }
        //         last_value = Vec::new();
        //     }
        // };

        for arg in args {
            let mut long_opt: i32 = -1;
            let mut shot_opt: i32 = -1;
            if let Some(vi) = arg.find("--") {
                long_opt = vi as i32;
            };
            if let Some(vi) = arg.find("-") {
                shot_opt = vi as i32;
            };
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
                    if !last_value.is_empty() {
                        if !data.contains_key(last_opt.as_str()) {
                            data.insert(last_opt, last_value);
                        }
                        last_value = Vec::new();
                    }
                    last_opt = String::from(opt);
                    option.push(String::from(&last_opt));
                } else if shot_opt == 0 {
                    let (_, opt) = arg.split_at(1);
                    let mut short_count = 0;
                    for by in opt.chars() {
                        last_opt = String::from(by);
                        if !last_value.is_empty() && short_count == 0 {
                            if !data.contains_key(last_opt.as_str()) {
                                data.insert(last_opt, last_value);
                            }
                            last_value = Vec::new();
                        }
                        option.push(String::from(&last_opt));
                        short_count += 1;
                    }
                } else {
                    last_value.push(String::from(arg));
                }
            }
            count += 1;
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
