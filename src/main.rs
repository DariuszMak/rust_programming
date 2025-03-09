mod counter;

fn main() {
    let counter = counter::Counter::new();
    counter.run_threads(100000);
    println!("Final counter value: {}", counter.get_value());
}
