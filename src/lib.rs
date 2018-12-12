// 版本信息
pub mod uymas {
    pub const VERSION: &'static str = "0.1.0-alpha";
    pub const RELEASE: &'static str = "20181112";
}


#[cfg(test)]
mod lib_test{
    #[test]
    fn base(){
        //print!("{}", uymas::VERSION);
        print!("{}", 99);
    }
}

#[cfg(test)]
mod decimal_test{}
