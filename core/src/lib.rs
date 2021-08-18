// 版本信息
pub const VERSION: &'static str = "0.1.0-alpha";
pub const RELEASE: &'static str = "2019-06-18";
pub const SINCE: &'static str = "2018-11-12";
pub const CODE: &'static str = "Uymas";
pub const TITLE: &'static str = "Rust 工具包";

//包列表
pub mod bin;
pub mod util;

// 公共函数
pub fn std_path(vp: &str) -> String {
    let mut v1 = String::from(vp);
    v1 = v1.replace("\\\\", "/");
    v1 = v1.replace("\\", "/");
    v1
}

// 程序测试
// #[cfg(test)]
// mod lib_test {
//     #[test]
//     fn base() {}
// }

mod lib_test;

#[cfg(test)]
mod decimal_test {
    #[test]
    fn base() {}
}
