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

/// `Experimental` 实验性特性。
/// 用于定义二进制应用
#[cfg(feature = "cli-derive")]
pub trait CliApp {
    /// 运行命令行程序
    /// ```
    /// use uymas_cli::CliApp;
    /// use uymas_cli::derive::CliApp;
    ///
    /// #[derive(CliApp)]
    /// struct MyApp;
    ///
    /// fn main(){
    ///     let app = MarcoApp{};
    ///     app.run();
    /// }
    /// ```
    fn run(&self);
}

/// 引入 `cli-derive` 实现调用过程宏 `Experimental`
#[cfg(feature = "cli-derive")]
pub use cli_derive as derive;
