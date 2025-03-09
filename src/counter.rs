use std::sync::{Arc, Mutex};
use std::thread;

pub struct Counter {
    value: Arc<Mutex<i32>>,
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: Arc::new(Mutex::new(0)),
        }
    }

    #[allow(dead_code)]
    pub fn increment(&self) {
        let mut num = self.value.lock().unwrap();
        *num += 1;
    }

    pub fn get_value(&self) -> i32 {
        *self.value.lock().unwrap()
    }

    pub fn run_threads(&self, thread_count: usize) {
        let mut handles = vec![];

        for _ in 0..thread_count {
            let counter_clone = Arc::clone(&self.value);
            let handle = thread::spawn(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
