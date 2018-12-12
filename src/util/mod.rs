// decimal 十进制处理
// 数字格式
pub const NUMBER_STR: &'static str = "0123456789abcdefghijklmnopqrstuvzxyzABCDEFGHIJKLMNOPQRSTUVWXYZ+-=";
pub const NSTR_LEN: usize = NUMBER_STR.len();


pub struct Decimal{
    num: i32,
}

// 十进制测试
impl Decimal{
    pub fn new(num: i32) -> Decimal {
        Decimal{num}
    }
    // 数据转换
    pub fn to_n(&self, base: i32) -> String{
        // 限制最大长度
        if base >= NSTR_LEN{
            return String::from("");
        }
        let mut num = self.num;
        let mut bque: Vec<i32> = vec![];
        let mut value: String;
        loop {
            if num < base{
                bque.push(num);
                break;
            }
            let md = num % base;
            bque.push(md);
            num = (num - md)/base;
        }
        // 数据收集
        //let mut sque: Vec<str> = vec![];
        let mut sque:Vec<str> = Vec::new();
        let mut i = (bque.len() - 1) as i32;
        let mut ref_que:&mut Vec<u8>;
        // 非安全代码
        unsafe {
            ref_que = String::from(NUMBER_STR).as_mut_vec();
        }

        while i > -1{
            let index = bque[i as usize];
            sque.push(ref_que[index]);
            i -= 1;
        }
        //value = sque.join(&String::from(""))
        sque.join("")
    }
}

#[cfg(test)]
mod test{
    use super::{Decimal};

    #[test]
    fn new(){
        assert_eq!("", Decimal::new(100).to_n(10))
    }
}