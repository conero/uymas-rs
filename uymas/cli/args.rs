use crate::cmd::get_os_args;
use std::collections::HashMap;
use std::env;
use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// 命令行解析后的参数
#[derive(Debug, Default)]
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
    pub is_extern_subc: bool, // 外部子命令
}

impl Args {
    /// 获取命令行参数，支持数组读取
    /// ```
    /// use uymas_cli::args::Args;
    /// use uymas_cli::cmd::{Cmd, CmdRunOs, CmdRunStr};
    /// let arg = Args::from_vec(vec!["app", "--options", "A", "B", "C c", "D", "-xty", "1", "2", "3", "4"]);
    /// assert_eq!(arg.get_data("options"), Some(&vec!["A".to_string(), "B".to_string(), "C c".to_string(), "D".to_string()]));
    /// assert_eq!(arg.get_data("y"), Some(&vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()]));
    ///
    /// ```
    /// **get_data** 下个版本调整 key 参数类型为 Vec<&str>
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
    #[deprecated(
        since = "2.0.1",
        note = "use the `get_option_string` instead, v2.2.x will remove"
    )]
    pub fn get_value_string_option(&self, keys: Vec<&str>) -> Option<String> {
        self.get_option_string(keys)
    }

    /// 获取 i32 数据类型
    pub fn get_value_i32(&self, keys: Vec<&str>) -> i32 {
        if let Some(v) = self.get_option::<i32>(keys) {
            return v;
        }
        0
    }

    /// 获取 bool 类型数据
    pub fn get_value_bool(&self, keys: Vec<&str>) -> bool {
        if let Some(v) = self.get_option::<bool>(keys) {
            v
        } else {
            false
        }
    }

    /// 获取 bool 类型数据
    pub fn get_value_f64(&self, keys: Vec<&str>) -> f64 {
        if let Some(v) = self.get_option(keys) {
            v
        } else {
            0.0
        }
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

    // 参数解析处理过程参数
    fn _args_update_data(
        data: &HashMap<String, Vec<String>>,
        last_opt: &str,
        last_value: &[String],
    ) -> (HashMap<String, Vec<String>>, Vec<String>) {
        let mut new_data = data.clone();
        let mut new_value = last_value.to_owned();

        if !last_opt.is_empty() && !last_value.is_empty() {
            if data.contains_key(last_opt) {
                let mut exits_value = data.get(last_opt).unwrap().to_vec();
                for lv in new_value {
                    exits_value.push(lv);
                }
                new_data.insert(last_opt.to_owned(), exits_value.to_vec());
            } else {
                new_data.insert(last_opt.to_owned(), new_value);
            }
            new_value = Vec::new();
        }
        (new_data, new_value)
    }

    /// 实例化函数，解析桉树为命令行。`Cmd.data` 解析参照 url.Values 解析规则，即默认为 `Vec<String>`
    /// ```
    /// // 解析 "rustc --version" 命令
    /// use uymas_cli::cmd::{Cmd, CmdRunOs, CmdRunStr};
    /// let cmd = Cmd::new();
    /// cmd.run(vec!["rustc", "--version"]);
    /// ```
    pub fn from_args(args: &Vec<String>) -> Self {
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
            if arg.is_empty() {
                continue;
            }
            let mut long_opt: i32 = -1;
            let mut shot_opt: i32 = -1;
            let mut equal_opt: usize = 0; // 是否含等于
            if let Some(vi) = arg.find("--") {
                long_opt = vi as i32;
            } else if let Some(vi) = arg.find('-') {
                shot_opt = vi as i32;
            };
            if let Some(vi) = arg.find('=') {
                equal_opt = vi;
            }

            let is_not_opt = shot_opt != 0 && long_opt != 0;
            if count == 0 && is_not_opt {
                // 命令解析
                command = String::from(arg);
            } else if count == 1 && !command.is_empty() && is_not_opt {
                // 子命令
                sub_command = String::from(arg);
            } else if !is_not_opt {
                let (ndt, nv) = Args::_args_update_data(&data, &last_opt, &last_value);
                data = ndt;
                last_value = nv;

                // 选项处理
                let is_log_opt = long_opt == 0;
                let split_idx = if is_log_opt { 2 } else { 1 };
                let (_, opt) = arg.split_at(split_idx);

                if is_log_opt {
                    last_opt = String::from(opt);
                } else {
                    for by in opt.chars() {
                        last_opt = String::from(by);
                        if !last_opt.contains('=') {
                            option.push(String::from(&last_opt));
                        }
                    }
                }

                if let Some(vi) = last_opt.find('=') {
                    equal_opt = vi;
                }
                if equal_opt > 0 {
                    // 处理含等于符号的选项
                    let (eq_key, eq_value) = last_opt.split_at(equal_opt);
                    last_value.push(String::from(&eq_value[1..]));
                    last_opt = String::from(eq_key);
                }

                option.push(String::from(&last_opt));
            } else {
                last_value.push(String::from(arg));
            }
            count += 1;
        }

        // 临界值（最后出现的）
        let (ndt, _) = Args::_args_update_data(&data, &last_opt, &last_value);
        data = ndt;

        Args {
            command,
            sub_command,
            option,
            data,
            raw: args.to_vec(),
            is_extern_subc: false,
        }
    }

    /// 使用参数命令行列表
    pub fn from_vec(args: Vec<&str>) -> Self {
        let v_list: Vec<String> = args.iter().map(|v| String::from(*v)).collect();
        Self::from_args(&v_list)
    }

    /// 根据 os::args 获取参数
    pub fn from_os() -> Self {
        Args::from_args(&get_os_args())
    }

    /// 根据字符串转 Args 参数
    pub fn from_str(param: &str) -> Self {
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
    #[deprecated(
        since = "2.1.0",
        note = "use the `get_option` instead, v2.2.x will remove"
    )]
    pub fn get_option_isize(&self, keys: Vec<&str>) -> Option<isize> {
        self.get_option(keys)
    }

    /// 获取 bool 类型
    #[deprecated(
        since = "2.1.0",
        note = "use the `get_option` instead, v2.2.x will remove"
    )]
    pub fn get_option_bool(&self, keys: Vec<&str>) -> Option<bool> {
        self.get_option(keys)
    }

    /// 获取 f64 类型
    #[deprecated(
        since = "2.1.0",
        note = "use the `get_option` instead, v2.2.x will remove"
    )]
    pub fn get_option_f64(&self, keys: Vec<&str>) -> Option<f64> {
        self.get_option(keys)
    }

    /// 获取任意解析参数
    pub fn get_option<T>(&self, keys: Vec<&str>) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        if let Some(raw) = self.get_option_string(keys) {
            let value_or_err = raw.parse::<T>();
            if let Ok(value) = value_or_err {
                return Some(value);
            }
        }
        None
    }

    /// 自定义构造 Args 对象并解析为字符串参数
    ///
    /// # Example
    ///
    /// 基本使用：
    /// ```
    /// use std::collections::HashMap;
    /// use uymas_cli::args::Args;
    /// let mut data = HashMap::from([
    ///     ("output".to_string(), vec!["./bin/name".to_string()]),
    ///     ("arch".to_string(), vec!["amd64".to_string()]),
    ///     ("x".to_string(), vec!["windows".to_string(), "unix".to_string()]),
    ///     ("stage".to_string(), vec!["b1".to_string(), "t2".to_string()]),
    /// ]);
    /// let app = Args{
    ///     command: "jc".to_string(),
    ///     sub_command: "build".to_string(),
    ///     option: vec!["utf8".to_string(), "release".to_string(), "mini".to_string(), "g".to_string()],
    ///     data,
    ///     raw: vec![],
    ///     is_extern_subc: false,
    /// };
    ///
    /// let ref_msg =
    ///     String::from("jc build --output ./bin/name --arch amd64 -x windows unix --stage b1 t2 --utf8 --release --mini -g");
    /// assert_eq!(app.parse_string(), ref_msg);
    /// ```
    pub fn parse_string(&self) -> String {
        let mut raw = vec![];
        // 命令行
        if !self.command.is_empty() {
            raw.push(self.command.clone());
            if !self.sub_command.is_empty() {
                raw.push(self.sub_command.clone());
            }
        }

        // 数据还原（复原）
        for (opt, ls) in &self.data {
            let name = if opt.len() == 1 {
                format!("-{}", opt)
            } else {
                format!("--{}", opt)
            };
            raw.push(format!("{} {}", name, ls.join(" ")));
        }

        // 选项复原
        for opt in self.option.clone().into_iter() {
            if self.data.contains_key(&opt) {
                continue;
            }
            let name = if opt.len() == 1 {
                format!("-{}", opt)
            } else {
                format!("--{}", opt)
            };
            raw.push(name);
        }

        raw.join(" ")
    }

    /// 获取原始输入指定选项（参数）的下一个值
    pub fn next(self, cur: String) -> Option<String> {
        let mut inx: i32 = -1;
        let mut cur_index = -1;
        for s in &self.raw {
            inx += 1;
            if *s == cur {
                cur_index = inx + 1;
                break;
            }
        }

        let size = self.raw.len();
        if cur_index > -1 && cur_index < size as i32 {
            return Some(self.raw[cur_index as usize].clone());
        }
        None
    }

    /// 获取原始输入指定选项（参数）的上一个值
    pub fn prev(self, cur: String) -> Option<String> {
        let mut inx: i32 = -1;
        let mut cur_index = -1;
        for s in &self.raw {
            inx += 1;
            if *s == cur {
                cur_index = inx - 1;
                break;
            }
        }

        let size = self.raw.len();
        if cur_index > -1 && cur_index < size as i32 {
            return Some(self.raw[cur_index as usize].clone());
        }
        None
    }
}

pub trait ArgsFromOs {
    fn new() -> Self;
}

impl ArgsFromOs for Args {
    fn new() -> Self {
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
        Args::from_args(param)
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

impl Clone for Args {
    fn clone(&self) -> Self {
        Args {
            command: self.command.clone(),
            sub_command: self.sub_command.clone(),
            option: self.option.clone(),
            data: self.data.clone(),
            raw: self.raw.clone(),
            is_extern_subc: self.is_extern_subc,
        }
    }
}

/// 可配置的 option解析
pub trait ArgsOptionParseDefine {
    fn get_option_vec<T>(&self, keys: Vec<&str>, split: &str) -> Option<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug;
}

impl ArgsOptionParseDefine for Args {
    fn get_option_vec<T>(&self, keys: Vec<&str>, split: &str) -> Option<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        if let Some(v_str) = self.get_option_string(keys) {
            return parse_option_vec(v_str, split);
        }
        None
    }
}

pub trait ArgsOptionParse {
    fn get_option_vec<T>(&self, keys: Vec<&str>) -> Option<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug;
}

impl ArgsOptionParse for Args {
    fn get_option_vec<T>(&self, keys: Vec<&str>) -> Option<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        if let Some(v_str) = self.get_option_string(keys) {
            return parse_option_vec(v_str, ",");
        }
        None
    }
}

/// 获取二进制项目(当前应用)所在目录
/// ### example
/// ```
///  // ~/uymas.exe
///  use uymas_cli::args::project_path;
///  println!("{}", project_path("logs/2023/05/27.log"));
///  // 输入: `~/logs/2023/05/27.log`
/// ```
pub fn project_path<P: AsRef<Path>>(joins: P) -> String {
    let path = base_bin_path(joins);
    path.replace('\\', "/")
}

/// 获取当前正在执行二进制所在路径
pub fn get_exec_path() -> String {
    let mut exec_path = String::new();
    if let Some(arg) = env::args().next() {
        exec_path = arg;
    }

    exec_path.replace('\\', "/")
}

// 通过解析函数获取当前目录地址
fn get_exec_dir_by_args() -> String {
    let exec_path = get_exec_path();
    let pth = Path::new(&exec_path);

    if let Some(parent) = pth.parent() {
        parent.to_str().unwrap().to_string()
    } else {
        String::new()
    }
}

/// 获取当前正在执行二进制所在目录
pub fn get_exec_dir() -> String {
    let mut v_dir = get_exec_dir_by_args();
    if v_dir.is_empty() {
        v_dir = if let Ok(cur_dir) = env::current_dir() {
            if let Some(v) = cur_dir.to_str() {
                String::from(v)
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    v_dir
}

/// 获取当前正在执行二进制名称
pub fn get_exec_name() -> String {
    let exec_path = get_exec_path();
    let pth = Path::new(&exec_path);

    if let Some(vfl) = pth.file_name() {
        vfl.to_str().unwrap().to_string()
    } else {
        String::new()
    }
}

/// 过去可执行名称无后缀
pub fn get_exec_no_ext() -> String {
    let exec_path = get_exec_path();
    let pth = Path::new(&exec_path);

    if let Some(vfl) = pth.file_name() {
        let mut file_name = vfl.to_str().unwrap().to_string();
        if let Some(ext) = pth.extension() {
            file_name = file_name.replace(&ext.to_str().unwrap().to_string(), "");
        }
        file_name
    } else {
        String::new()
    }
}

/// 根据当前正在执行二进制所在吗，目录进行 split
pub fn root_path_split() -> (String, String) {
    let exec_path = get_exec_path();
    let pth = Path::new(&exec_path);

    let file_name = if let Some(vfl) = pth.file_name() {
        vfl.to_str().unwrap().to_string()
    } else {
        String::new()
    };

    if let Some(parent) = pth.parent() {
        (parent.to_str().unwrap().to_string(), file_name)
    } else {
        (String::new(), file_name)
    }
}

// 依赖二进制所在目录
fn base_bin_path<P: AsRef<Path>>(joins: P) -> String {
    let basedir = root_path_split();
    let pth = Path::new(basedir.0.as_str());
    let pth = pth.join(joins);
    pth.to_str().unwrap().to_string()
}

/// 参数解析
fn parse_option_vec<T>(value: String, split: &str) -> Option<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let vec_list: Vec<&str> = value.split(split).collect();
    let mut parse = Vec::new();
    for vs in vec_list {
        let rslt_or_err = String::from(vs).parse::<T>();
        if let Ok(relt) = rslt_or_err {
            parse.push(relt);
        }
    }

    if parse.is_empty() {
        return None;
    }

    Some(parse)
}

/// 获取当前系统运行的目录
pub fn get_current_dir<P: AsRef<Path>>(v_path: P) -> Result<PathBuf, std::io::Error> {
    let cur_dir = env::current_dir()?;
    Ok(cur_dir.join(v_path))
}
