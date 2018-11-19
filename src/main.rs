// @date 2018年11月16日 星期五
// @author Joshua
use std::env;
use std::process::{Command, Stdio};

// 系统常量
const VERSION: &'static str = "1.0.1";
const RELEASE: &'static str = "20181119";

// 运行名字
fn name_exec(s: &String) -> String{
    let s = s.as_str().replace("\\", "/");
    let v: Vec<&str> = s.as_str().split("/").collect();
    let mut name = format!("{}", v.last().expect("_test"));
    name = name.to_lowercase();
    name = name.replace(".rs", "");
    name
}


// 运行 fp 文件
fn exec(fp: String){
    println!(" compiler the file: {}", fp);
    let fname:String = name_exec(&fp);

    Command::new("rustc")
        .arg(fp)
        .arg("-o")
        .arg(&format!("{}.exe", fname))
        .output()
        .expect("failed to execute process")
        ;

    // 字符互处理十分的麻烦
    let exc_name = format!("./{}.exe", fname);
    println!(" will run the applicaton: {}", exc_name);
    let app = Command::new(&exc_name)
        //.output()
        .stdout(Stdio::piped())
        .output()
        .expect(&format!("{} 运行失败", exc_name))
    ;

    // 输出运行结果
    println!(" status: {}", app.status);
    println!(" stdout: \n------------------(Uymas Rustc)-------------------\n\n{}", String::from_utf8_lossy(&app.stdout));
}

fn main() {
    let mut ctt = 0;
    for arg in env::args(){
        ctt += 1;
        if ctt == 1{
            continue;
        }
        exec(arg)
    }

    // 未查找到数据时
    if ctt == 1{
        println!(" 欢迎使用 uyrsc v{}/{}", VERSION, RELEASE);
        println!(" Joshua Conero");
        println!(" [using]: uyrsc.exe <file>");


        // name_exec 函数测试
        //assert_eq!("type.rs".to_string(), );
        //println!("{}", name_exec(&"/type.rs.rs.RS".to_string()));
    }
}
