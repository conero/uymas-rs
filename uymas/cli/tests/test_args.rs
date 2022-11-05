// extern crate uymas_cli as cli;
// 执行测试用例: `cargo test --test test_args`
use uymas_cli::args::{Args, ArgsNew};

#[test]
fn args_contain_opts() {
    // case 1
    let input: Vec<String> = vec![String::from("rustc"), String::from("--version")];
    let args = <Args as ArgsNew<&Vec<String>>>::new(&input);
    assert_eq!(args.contain_opts(vec!["version"]), true);

    // case2
    let input: Vec<String> = vec![String::from("--version")];
    let args = Args::new(&input);
    assert_eq!(args.contain_opts(vec!["version"]), true);

    // case3
    let input = vec!["-Xyz"];
    let args = Args::new(input);
    assert_eq!(args.contain_opts(vec!["z"]), true);

    // case
    let input: Vec<String> = vec![
        String::from("git"),
        String::from("remote"),
        String::from("set-url"),
        String::from("--add"),
    ];
    let args = Args::new(input);
    assert_eq!(args.contain_opts(vec!["add"]), true);

    // case
    let args = Args::from_str("git log --stat");
    assert_eq!(args.contain_opts(vec!["stat", "test"]), true);
}

#[test]
fn parse() {
    let input: Vec<String> = vec![String::from("test-pool"), String::from("main")];
    let args = <Args as ArgsNew<&Vec<String>>>::new(&input);
    assert_eq!(args.command, "test-pool");

    // case
    let args = Args::from_str("git status");
    assert_eq!(args.sub_command, "status");
}

#[test]
fn test_get_value_string() {
    // case1
    let input: Vec<String> = vec![
        String::from("test-pool"),
        String::from("main"),
        String::from("--name"),
        String::from("joshua"),
    ];
    let args = <Args as ArgsNew<&Vec<String>>>::new(&input);
    assert_eq!(args.get_value_string(vec!["name"]), "joshua");

    // case2
    let ipt2 = vec!["git", "--name", "joshua", "Conero"];
    let args = <Args as ArgsNew<Vec<&str>>>::new(ipt2);
    assert_eq!(args.get_value_string(vec!["name"]), "joshua Conero");
}

#[test]
fn test_get_value_i32() {
    // case
    let args = Args::new(vec![
        "uymas", "--age", "2", "--year", "2022", "-xYZ", "1949",
    ]);
    assert_eq!(2, args.get_value_i32(vec!["age"]));
    assert_eq!(2022, args.get_value_i32(vec!["year"]));
    assert_eq!(1949, args.get_value_i32(vec!["Z"]));

    // case
    let args = Args::new(vec!["uymas", "--age=30", "--wight=100", "-xYZ", "11"]);
    assert_eq!(30, args.get_value_i32(vec!["age"]));
    assert_eq!(100, args.get_value_i32(vec!["wight"]));
    assert_eq!(0, args.get_value_i32(vec!["Z"]));
}
