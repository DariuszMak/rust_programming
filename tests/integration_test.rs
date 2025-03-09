use hello_world::counter::Counter;

#[test]
fn test_single_increment() {
    let counter = Counter::new();
    counter.increment();
    assert_eq!(counter.get_value(), 1);
}

#[test]
fn test_multithreading() {
    let counter = Counter::new();
    counter.run_threads(5);
    assert_eq!(counter.get_value(), 5);
}
