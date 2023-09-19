use crate::args;
use crate::args::project_path;
use std::process::Command;

/// 外部子命令
/// 检测优先级：假设当前app名称为 $app
/// - 同目录下- $name
/// - 同目录下- $app-$name
/// - plg/$name
/// - plg/$app-$name
/// - $app-$name     全局app
pub struct ExternSubc {
    name: String,
    command: String, // 实际执行命令
    is_valid: bool,  // 是否有效
    app_name: String,
    search_list: Vec<String>,
}

impl ExternSubc {
    /// 初始化
    pub fn new(name: String) -> Self {
        let mut esc = Self {
            name,
            command: String::new(),
            is_valid: false,
            app_name: args::get_exec_no_ext(),
            search_list: vec![],
        };
        esc.detect();
        esc
    }

    /// 探测
    pub fn detect(&mut self) {
        self.search_list = vec![];
        // ./$name
        let command = project_path(self.name.clone());
        self.search_list.push(command.clone());
        let cmd_rslt = Command::new(command.clone()).output();
        if cmd_rslt.is_ok() {
            self.is_valid = true;
            self.command = command;
            return;
        }

        // ./$app-$name
        let app = self.app_name.clone();
        self.search_list.push(command.clone());
        let command = project_path(format!("{}-{}", app, self.name.clone()));
        let cmd_rslt = Command::new(command.clone()).output();
        if cmd_rslt.is_ok() {
            self.is_valid = true;
            self.command = command;
            return;
        }

        // ./plg/$name
        let command = project_path(format!("plg/{}", self.name.clone()));
        self.search_list.push(command.clone());
        let cmd_rslt = Command::new(command.clone()).output();
        if cmd_rslt.is_ok() {
            self.is_valid = true;
            self.command = command;
            return;
        }

        // ./plg/$app-$name
        let app = self.app_name.clone();
        self.search_list.push(command.clone());
        let command = project_path(format!("plg/{}-{}", app, self.name.clone()));
        let cmd_rslt = Command::new(command.clone()).output();
        if cmd_rslt.is_ok() {
            self.is_valid = true;
            self.command = command;
            return;
        }

        self.is_valid = false;
    }

    /// 运行命令
    pub fn run(&self, args: &Vec<String>) -> (bool, String) {
        if !self.is_valid {
            return (false, String::new());
        }
        let mut run = Command::new(self.command.clone());
        for arg in args {
            run.arg(arg);
        }
        if let Ok(rslt) = run.output() {
            return (true, String::from_utf8(rslt.stdout).unwrap());
        }

        (false, String::new())
    }

    /// 获取搜索列表
    pub fn get_search(&self) -> Vec<String> {
        self.search_list.clone()
    }

    /// 是否有效
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}
