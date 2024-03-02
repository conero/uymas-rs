use std::time::Instant;

/// 获取到程序运行消耗的时间秒
pub fn spend_time_diff() -> impl Fn() -> f64 {
    let now = Instant::now();
    move || now.elapsed().as_secs_f64()
}
