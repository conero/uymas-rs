extern crate proc_macro;

use proc_macro::TokenStream;

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

#[cfg(feature = "cli-derive")]
#[proc_macro_derive(CliApp)]
pub fn cli_app(input: TokenStream) -> TokenStream {
    input
}

#[cfg(feature = "cli-derive")]
#[proc_macro_attribute]
pub fn cli_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[cfg(feature = "cli-derive")]
#[proc_macro_attribute]
pub fn cli_action(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// 命令程序映射
#[cfg(feature = "cli-derive")]
pub mod reflect;

// 测试用例，使用 tests 代替
// #[cfg(test)]
// mod test_args;
