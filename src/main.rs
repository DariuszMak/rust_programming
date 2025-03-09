mod counter;

fn main() {
    let counter = counter::Counter::new();
    counter.run_threads(10);
    println!("Final counter value: {}", counter.get_value());
}
