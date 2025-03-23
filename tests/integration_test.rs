use hello_world::Clock; // Use the crate name in integration tests
use std::time::Duration;

#[test]
fn test_clock_integration() {
    let time_before = Clock::now();
    Clock::tick(Duration::from_secs(1));
    let time_after = Clock::now();

    assert!(time_after >= time_before + 1);
}
