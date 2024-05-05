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

/// 不直接使用 `ErrMsgString`，其作为内部的参数使用
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
    /// 抛出`String`异常消息
    /// # Examples
    ///
    /// 基本使用
    ///
    /// ```
    /// use uymas_cli::err::ErrMsg;
    /// fn to_throw_err() -> Result<(), Box<dyn std::error::Error>>{
    ///     Err(ErrMsg::throw("参数缺少".to_string()))
    /// }
    /// ```
    pub fn throw(msg: String) -> Box<dyn Error> {
        Box::from(<ErrMsg as ErrMsgString>::new(msg))
    }

    /// 抛出`str`异常消息
    /// # Examples
    ///
    /// 基本使用
    ///
    /// ```
    /// use uymas_cli::err::ErrMsg;
    /// fn to_throw_err() -> Result<(), Box<dyn std::error::Error>>{
    ///     Err(ErrMsg::throw_str("请求失败"))
    /// }
    /// ```
    pub fn throw_str(msg: &str) -> Box<dyn Error> {
        Box::from(<ErrMsg as ErrMsgString>::new(String::from(msg)))
    }
}
