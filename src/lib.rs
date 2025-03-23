use std::thread::sleep;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

/// A simple clock struct
pub struct Clock;

impl Clock {
    /// Returns the current time as a UNIX timestamp
    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }

    /// Simulates ticking by sleeping for a given duration
    pub fn tick(duration: Duration) {
        sleep(duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_now() {
        let time1 = Clock::now();
        let time2 = Clock::now();
        assert!(time2 >= time1); // Time should never go backwards
    }

    #[test]
    fn test_clock_tick() {
        let start = Clock::now();
        Clock::tick(Duration::from_secs(1));
        let end = Clock::now();
        assert!(end >= start + 1);
    }
}
