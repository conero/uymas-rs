use std::time::Instant;

/// 获取到程序运行消耗的时间秒
/// ### Example
/// ```
///     use std::{thread, time};
///     use uymas_cli::util::spend_time_diff;
///
///     let spend_time_fn = spend_time_diff();
///     let sleep_sec = time::Duration::from_secs_f32(4.243f32);
///     thread::sleep(sleep_sec);
///     println!("time spend: {:.3}s", spend_time_fn())
/// ```
pub fn spend_time_diff() -> impl Fn() -> f64 {
    let now = Instant::now();
    move || now.elapsed().as_secs_f64()
}
