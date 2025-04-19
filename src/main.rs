use std::time::Duration;
use std::thread;

fn main() {
    println!("Hello, world!");
    thread::sleep(Duration::from_millis(500))
    println!("Bye, world!");
}
