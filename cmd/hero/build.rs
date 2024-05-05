// 2023年8月7日 星期一 构建程序
// build.rs

use std::path::Path;
use std::process::Command;
use std::{env, fs};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    // 生成文件
    let dest_path = Path::new(&out_dir).join("uymas-version.rs");

    // 系统时间
    let time_str = chrono::Local::now().format("%Y%m%d").to_string();
    let hash_str = get_curr_hash();

    let meta_cmd = cargo_metadata::MetadataCommand::new();
    let metadata = meta_cmd.exec().expect("meta 执行错误");
    let package = metadata
        .packages
        .into_iter()
        .find(|p| p.name == "hero")
        .unwrap();

    let mut content = "
        pub fn build_git_hash() -> &'static str{
            \""
    .to_owned()
        + &hash_str
        + "\"
        }";

    content += &*("
        pub fn build_version() -> &'static str{
            \""
    .to_owned()
        + &package.version.to_string()
        + "\"
        }
    ");

    content += &*("
        pub fn build_date() -> &'static str{
            \""
    .to_owned()
        + &time_str
        + "\"
        }
    ");
    // 文件写入
    fs::write(dest_path, content).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

fn get_curr_hash() -> String {
    // git rev-parse HEAD
    let cmd = Command::new("git").args(["rev-parse", "HEAD"]).output();
    if let Ok(cmd_ok) = cmd {
        let output = String::from_utf8(cmd_ok.stdout).unwrap();
        let output = String::from(&output[..10]);
        output.trim().to_string()
    } else {
        "Err Cmd".to_string()
    }
}
