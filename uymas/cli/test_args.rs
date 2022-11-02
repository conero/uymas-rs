use crate::args::{Args, ArgsNew};
use std::env;

#[test]
fn cmd_from() {
    //let app = Cmd::from(vec![]);
    println!("{:?}", env::args());

    for argument in env::args() {
        println!("{argument}");
    }
}

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
