pub use std::sync::Mutex;
pub use std::thread;

pub struct Counter {
    pub value: usize,
}


//private, one instance, fixed memory in data segment
static COUNTER: Mutex<Counter> = Mutex::new(Counter { value: 0 });

impl Counter {
    //returning ref to STATIC variable, so that no need to use Arc<Mutex<Counter>> and can be used directly in multiple threads
    pub fn get_counter() -> &'static Mutex<Counter> {
        &COUNTER
    }
}

#[cfg(test)]
mod singleton_tests {
    use super::*;

    #[test]
    fn initial() {
        let counter = Counter::get_counter();
        assert_eq!(0, counter.lock().unwrap().value);
    }

    #[test]
    fn increment() {
        let counter = Counter::get_counter();
        let mut counter = counter.lock().unwrap(); //Mutex has interior mutability, doesnt mean that inner value should be mutable
        counter.value += 1;
        assert_eq!(1, counter.value);
    }

    #[test]
    #[ignore]
    fn increment_threads() {
        let counter = Counter::get_counter();

        let handles: Vec<_> = (0..10)
            .map(|_| {
                thread::spawn(move || {
                    let mut counter = counter.lock().unwrap();
                    counter.value += 1;
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let counter = counter.lock().unwrap();
        assert_eq!(10, counter.value);
    }
}
