// @date 2018年11月16日 星期五
// @author Joshua
extern crate rand;

use std::env;
use uyrsc::*;


fn main() {
    let mut ctt = 0;
    let mut is_o = false;
    for arg in env::args(){
        ctt += 1;
        if ctt == 1{
            continue;
        }
        if arg.as_str() == "-o"{
            is_o = true;
            continue;
        }
        if is_o{
            uyrsc::flag_o::exec(arg);
        }else {
            uyrsc::flag_dir::exec(arg);
        }

    }

    // 未查找到数据时
    if ctt == 1{
        println!(" Welcome to uyrsc v{}/{}", jc::VERSION, jc::RELEASE);
        println!(" Power by {}", jc::AUTHOR);
        println!(" Find Me Like Email: {}", jc::EMAIL);
        println!();
        println!(" [using]: uyrsc.exe [<options>] <file>");
        println!("          -o   use the rustc flag -o");
        println!("          default flag: --out-dir");
        // name_exec 函数测试
        //assert_eq!("type.rs".to_string(), );
        //println!("{}", name_exec(&"/type.rs.rs.RS".to_string()));

        // Try printing a random unicode code point (probably a bad idea)!
    }
}


mod uyrsc;