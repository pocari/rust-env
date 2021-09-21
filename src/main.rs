use std::env;

fn main() {
    let hoge = env::var("HOGE").unwrap_or("(none)".to_string());
    println!("hoge ... {}", hoge)
}
