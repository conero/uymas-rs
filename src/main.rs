// @date 2018年11月16日 星期五
// @author Joshua
use std::env;
use std::process::Command;

fn exec(fp: String){
    println!("{}", fp);
    let fname:String = String::from("test");

    Command::new("rustc")
        .arg(fp)
        .arg("-o")
        .arg(&fname)
        .output()
        .expect("failed to execute process")
        ;
    // 字符互处理十分的麻烦
    let exc_name = format!("./{}", fname);
    Command::new(exc_name)
        .output()
        .expect("failed to execute process")
    ;

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
        println!(" 欢迎使用 uyrsc");
        println!(" Joshua Conero");
        println!(" [using]: uyrsc.exe <file>");
    }
}
