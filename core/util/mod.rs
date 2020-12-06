// decimal 十进制处理
// 数字格式
pub const NUMBER_STR: &'static str =
    "0123456789abcdefghijklmnopqrstuvzxyzABCDEFGHIJKLMNOPQRSTUVWXYZ+-=";

pub struct Decimal {
    num: i32,
}

// 十进制转换
impl Decimal {
    pub fn new(num: i32) -> Decimal {
        Decimal { num }
    }

    // 【求整取余法】进制转换
    pub fn to_n(&self, base: i32) -> String {
        // 限制最大长度
        let max_strlen = NUMBER_STR.len() as i32;
        if base >= max_strlen {
            return String::from("");
        }

        // 参照队列
        let ref_que: Vec<char> = NUMBER_STR.chars().collect();
        let mut val_que: Vec<String> = vec![];

        // 求的所有余数
        let mut num = self.num;
        loop {
            let i = num % base;
            num = (num - i) / base;
            val_que.push(ref_que[i as usize].to_string());
            if num <= 0 {
                break;
            }
        }
        // 反转顺序
        let vque: Vec<String> = val_que.into_iter().rev().collect();
        vque.join(&"")
    }
}

#[cfg(test)]
mod test {
    use super::Decimal;

    #[test]
    fn new() {
        assert_eq!("1100011", Decimal::new(99).to_n(2));
        assert_eq!("143", Decimal::new(99).to_n(8));
        assert_eq!("63", Decimal::new(99).to_n(16));
    }
}
