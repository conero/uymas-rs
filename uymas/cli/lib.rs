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
/// 基本异常处理，字符串异常处理，使其兼容于常用的自定义错误信息
/// ```
/// // 自定义字符串异常抛出
/// use uymas_cli::err::ErrMsg;
///
/// // 自定义异常处理函数
/// fn test_err() -> Result<(), Box<dyn std::error::Error>>{
///     let content = std::fs::read_to_string("./file-no-exits.txt")?;
///     Err(ErrMsg::throw_str("自定义错误"))
/// }
///
/// // 错误无测试
/// let err_msg = test_err();
/// assert!(err_msg.is_err());
/// ```
pub mod err;
/// fs文件操作助手
pub mod util_fs;

// 测试用例，使用 tests 代替
// #[cfg(test)]
// mod test_args;
