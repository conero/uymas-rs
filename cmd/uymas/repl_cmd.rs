// 注入式命令

use cli::args::Args;

pub struct ReplCmd {
    arg: Option<Args>,
}

impl ReplCmd {
    pub fn new() -> ReplCmd {
        ReplCmd { arg: None }
    }

    // 执行
    pub fn run(&mut self, arg: &Args) {
        self.arg = Some(*arg);
    }
}
