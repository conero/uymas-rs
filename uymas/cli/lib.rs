///  依赖库版本信息库
pub const VERSION: &str = "2.1.1";
/// 项目代码
pub const PROJECT: &str = "uymas";
/// 发布日期，可选"dev"或"20060102"
pub const RELEASE: &str = "dev";

/// 命令行可注册命令
pub mod action;
/// 命令行参数
pub mod args;
/// 命令行实例
pub mod cmd;

// 测试用例，使用 tests 代替
// #[cfg(test)]
// mod test_args;
