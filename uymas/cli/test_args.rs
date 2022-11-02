use crate::args;
use crate::args::{Args, ArgsFromOs, ArgsValueVecStr, ArgsValueVecString};
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
    let args = <&Vec<String> as ArgsValueVecString>::new(&input);
    assert_eq!(args.contain_opts(vec!["version"]), true);

    // case2
    let input: Vec<String> = vec![String::from("--version")];
    let args = <&Vec<String> as ArgsValueVecString>::new(&input);
    assert_eq!(args.contain_opts(vec!["version"]), true);
}

#[test]
fn parse() {
    let input: Vec<String> = vec![String::from("test-pool"), String::from("main")];
    let args = <&Vec<String> as ArgsValueVecString>::new(&input);
    assert_eq!(args.command, "test-pool");
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
    let args = <&Vec<String> as ArgsValueVecString>::new(&input);
    assert_eq!(args.get_value_string(vec!["name"]), "joshua");

    // case2
    let ipt2 = vec!["git", ""];
    let args = <args::Args as ArgsValueVecStr>::new(ipt2);
    assert_eq!(args.get_value_string(vec!["name"]), "joshua");
}
