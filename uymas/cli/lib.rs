///  依赖库版本信息库
pub const VERSION: &'static str = "2.0.2";
/// 项目代码
pub const PROJECT: &'static str = "learn";

/// 命令行可注册命令
pub mod action;
/// 命令行参数
pub mod args;
/// 命令行实例
pub mod cmd;

// 测试用例，使用 tests 代替
// #[cfg(test)]
// mod test_args;
