//bin 命令程序包
//2019年6月18日 星期二
//Joshua Conero

/// `Router` 的基本使用
///
/// # Example
///
/// ```rust
/// use uymas::bin::router::Router;
/// let app = Router::new();
/// app.register("app");
/// app.run();      // 运行命运
/// ```
// 模块列表
mod router;

// 项目测试
#[cfg(test)]
mod router_test {
    use crate::bin::router::Router;

    #[test]
    fn base_new() {
        let app = Router::new();
        app.run();
        assert_eq!(1, 2)
    }
}
