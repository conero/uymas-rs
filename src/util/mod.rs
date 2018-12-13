// decimal 十进制处理
// 数字格式
pub const NUMBER_STR: &'static str = "0123456789abcdefghijklmnopqrstuvzxyzABCDEFGHIJKLMNOPQRSTUVWXYZ+-=";


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
        let max_strlen = NUMBER_STR.len() as i32;
        if base >= max_strlen{
            return String::from("");
        }
        let mut num = self.num;
        let mut bque: Vec<i32> = vec![];
        //let mut value: String;
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
        let mut sque: Vec<String> = vec![];
        //let mut sque:Vec<str> = Vec::new();
        let mut i = (bque.len() - 1) as i32;

        let ref_str = NUMBER_STR.get(..(base as usize));
        //let ref_que = format!("{}", ref_str).split("");
        //let ref_que = ref_str.expect(&"").split("");
        let ref_que:Vec<&str> = ref_str.unwrap().split("").collect();

        while i > -1{
            let index = (bque[i as usize]) as usize;
            sque.push(ref_que[index].to_string());
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
        //assert_eq!("1100011", Decimal::new(99).to_n(2));
        //assert_eq!("143", Decimal::new(99).to_n(8));
        assert_eq!("63", Decimal::new(99).to_n(16));
    }
}