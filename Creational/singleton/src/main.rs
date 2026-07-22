use singleton::Counter;
use singleton::thread;

fn main() {
    println!("Singleton Design Pattern!");

    //Check if same Instance is created
    let counter = Counter::get_counter();
    let counter1 = Counter::get_counter();
    assert!(std::ptr::eq(&*counter, &*counter1));
    println!("Address of counter in main binary is : {:p}", &*counter);
    println!("Address of counter1 in main binary is : {:p}", &*counter1);

    //Increment counter in multiple threads
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(move || {
                let mut counter = counter.lock().unwrap();
                counter.increment_value();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Counter value in main binary is : {}",
        counter.lock().unwrap().get_value()
    );
}
