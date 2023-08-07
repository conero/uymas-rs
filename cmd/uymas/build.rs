// 2023年8月7日 星期一 构建程序
// build.rs

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// 获取当前的 commit_id 命令 `git rev-parse HEAD`
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    // 生成文件
    let dest_path = Path::new(&out_dir).join("uymas-version.rs");

    // 系统时间
    let time_str = get_current_time();
    let hash_str = get_curr_hash();

    let mut content = "
        pub fn current_git_hash() -> &'static str{
            \""
    .to_owned()
        + &hash_str
        + "\"
        }";

    content += &*("
        pub fn build_date() -> &'static str{
            \""
    .to_owned()
        + &time_str
        + "\"
        }
    ");
    // 文件写入
    fs::write(&dest_path, content).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

// 获取当前时间
fn get_current_time() -> String {
    if let Ok(mt) = MTime::new() {
        format!("{}-{}-{}", mt.year, mt.month, mt.day)
    } else {
        "Time Err".to_string()
    }
}

struct MTime {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    minute: u64,
    second: u64,
}

impl MTime {
    pub fn new() -> Result<MTime, String> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(r) => Ok(timestamp_to_time(r)),
            Err(_) => Err("err".to_owned()),
        }
    }
}

fn timestamp_to_time(timestamp: Duration) -> MTime {
    let _second: u64 = timestamp.as_secs();
    let mut y: u64 = 1970;
    let mut m: u64 = 1;
    let mut d: u64 = (_second / (24 * 60 * 60)) + 1; //够24小时才会加，所以当天不够24小时并不会加上去。所以需要多加一天才会显示正确
    let s: u64 = _second % (24 * 60 * 60) % (60 * 60) % 60;
    let minute: u64 = (_second % (24 * 60 * 60)) % (60 * 60) / 60;
    let h: u64 = (_second % (24 * 60 * 60)) / (60 * 60) + 8;

    // 处理年月日
    loop {
        if d > 365 {
            // 大于365天，则判断年份
            if y % 4 == 0 && y % 100 != 0 || y % 400 == 0 {
                d -= 366;
            } else {
                d -= 365;
            }
            y += 1;
        } else {
            // 小于365天，则判断月份
            if d > 31 {
                d -= 31;
                m += 1;
                if m == 2 {
                    if y % 4 == 0 && y % 100 != 0 || y % 400 == 0 {
                        if d > 29 {
                            d -= 29;
                            m += 1;
                        }
                    } else {
                        if d > 28 {
                            d -= 28;
                            m += 1;
                        }
                    }
                } else if m == (3 | 5 | 7 | 8 | 10 | 12) {
                    if d > 31 {
                        d -= 31;
                        m += 1;
                    }
                } else {
                    if d > 30 {
                        d -= 30;
                        m += 1;
                    }
                }
            } else {
                break;
            }
        }
    }

    MTime {
        year: y,
        month: m,
        day: d,
        hour: h,
        minute: minute,
        second: s,
    }
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
