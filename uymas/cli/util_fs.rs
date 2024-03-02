use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// 目录检测是否存在，不存在时尝试创建
pub fn dir_check(dirname: String, is_parent: bool) -> bool {
    let mut pth = Path::new(dirname.as_str());
    if is_parent {
        if let Some(v_dir) = pth.parent() {
            pth = v_dir;
        } else {
            return false;
        }
    }
    if !pth.exists() {
        return fs::create_dir_all(pth).is_ok();
    }
    true
}

/// 文件追加，支持不存在的目录
pub fn append_file(fl_name: String, content: String) -> Result<bool, String> {
    // 获取父级，已查看目录是否存在不存在是创建
    dir_check(fl_name.clone(), true);

    let fl_rslt = OpenOptions::new()
        .create(true)
        .append(true)
        .open(fl_name.clone());

    match fl_rslt {
        Ok(mut fl) => {
            let w_rslt = fl.write_all(format!("{}\n", content).as_bytes());
            if w_rslt.is_ok() {
                Ok(true)
            } else {
                Err("文件写入错误".to_string())
            }
        }
        Err(er) => Err(format!("Error，{}日志写入失败，\n  {}", fl_name, er)),
    }
}
