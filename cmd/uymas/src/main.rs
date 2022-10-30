use core::bin::router::Router;

fn main() {
    println!("Hello, world!");
    println!("版本：{}", core::VERSION);
    let app = Router::new();
    app.run();
}
