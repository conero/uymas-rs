use core::bin::router::Router;
use core::VERSION;

fn main() {
    println!("Hello, world!");
    println!("版本：{}", core::VERSION);
    let app = Router::new();
    app.run();
}
