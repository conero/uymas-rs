use crate::args::Args;
use crate::cmd::Cmd;
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
    let args = Args::new(&input);
    assert_eq!(args.contain_opts(vec!["version"]), true);

    // case2
    let input: Vec<String> = vec![String::from("--version")];
    let args = Args::new(&input);
    assert_eq!(args.contain_opts(vec!["version"]), true);
}
