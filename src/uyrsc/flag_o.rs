use std::process::{Command, Stdio};

// 使用 rustc -o 参数
// 运行名字
pub fn name_exec(s: &String) -> String{
    let s = s.as_str().replace("\\", "/");
    let v: Vec<&str> = s.as_str().split("/").collect();
    let mut name = format!("{}", v.last().expect("_test"));
    name = name.to_lowercase();
    name = name.replace(".rs", "");
    name
}

// 运行 fp 文件
pub fn exec(fp: String){
    println!(" compiler the file: {}", fp);
    let fname:String = name_exec(&fp);

    // 编译
    let compiler = Command::new("rustc")
        .arg(fp)
        .arg("-o")
        .arg(&format!("{}.exe", fname))
        .output()
        .expect("failed to execute process")
    ;

    // 输出运行结果
    println!(" status: {}", compiler.status);
    println!(" stdout: \n------------------(Uymas Rustc)-------------------\n\n{}", String::from_utf8_lossy(&compiler.stdout));

    if !compiler.status.success(){
        println!(" (:_ the file compiled fail.");
        println!();
        return;
    }
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