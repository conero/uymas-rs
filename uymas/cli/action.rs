use crate::args::Args;

/// 可注册命令
pub trait Action {
    /// 初始化化
    fn init(&self) {}

    /// 执行方法入口
    fn run(&self, arg: &Args);
}
