use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct ErrMsg {
    msg: String,
}

impl Debug for ErrMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.msg.clone())
    }
}

impl Display for ErrMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.msg.clone())
    }
}

impl Error for ErrMsg {}

pub trait ErrMsgString {
    fn new(msg: String) -> Self;
}

pub trait ErrMsgStr {
    fn new(msg: &str) -> Self;
}

impl ErrMsgString for ErrMsg {
    fn new(msg: String) -> Self {
        ErrMsg { msg }
    }
}

impl ErrMsgStr for ErrMsg {
    fn new(msg: &str) -> Self {
        ErrMsg {
            msg: String::from(msg),
        }
    }
}

impl ErrMsg {
    // 错误抛出
    pub fn throw(msg: String) -> Box<dyn Error> {
        Box::try_from(<ErrMsg as ErrMsgString>::new(msg)).unwrap()
    }

    // 错误抛出
    pub fn throw_str(msg: &str) -> Box<dyn Error> {
        Box::try_from(<ErrMsg as ErrMsgString>::new(String::from(msg))).unwrap()
    }
}
