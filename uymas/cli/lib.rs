///  依赖库版本信息库
pub const VERSION: &'static str = "2.0.2";
/// 项目代码
pub const PROJECT: &'static str = "uymas";

/// 命令行可注册命令
pub mod action;
/// 命令行参数
pub mod args;
/// 命令行实例
pub mod cmd;

#[cfg(feature = "cli-derive")]
pub trait CliApp {
    fn run(); // 程序
}

/// 引入 `cli-derive` 实现调用过程宏
#[cfg(feature = "cli-derive")]
pub use cli_derive as derive;
