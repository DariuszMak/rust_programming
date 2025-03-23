use hello_world::Clock; // Use the crate name instead of `crate::Clock`
use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        println!("Current UNIX timestamp: {}", Clock::now());
        sleep(Duration::from_secs(1));
    }
}
