// decimal 十进制处理


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
        let mut sque = vec![];
        let mut i = (bque.len() - 1) as i32;
        while i > -1{
            sque.push(bque[i]);
            i -= 1;
        }
        value
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