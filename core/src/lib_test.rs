#[cfg(test)]
mod lib_test {
    use crate::VERSION;

    #[test]
    fn base() {
        print!("{}", VERSION);
    }
}

#[cfg(test)]
mod decimal_test {}

#[test]
fn test_std_path() {
    let mut name = r"conero\uymas-rs\target\debug\uymas.exe";
    let mut want = r"conero/uymas-rs/target/debug/uymas.exe";
    let mut real: String;

    // case 1
    real = crate::std_path(name);
    assert_eq!(real.as_str(), want);

    // case 2
    name = r"conero\uymas-rs\\target\debug\\uymas.exe";
    want = r"conero/uymas-rs/target/debug/uymas.exe";
    real = crate::std_path(name);
    assert_eq!(real.as_str(), want);
}
