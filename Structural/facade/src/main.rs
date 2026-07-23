use facade::facade::ThreadManager;
use facade::logger::ConsoleLogger;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let logger = Mutex::new(ConsoleLogger);

    //ThreadManager type is a facade to provide API to client and hide the complexity of thread management and logging.
    let manager = ThreadManager::new(Arc::new(logger));

    let mut handles = Vec::new();

    for i in 1..=5 {
        let handle = manager.spawn(format!("Client Request from-{i}"), move || {
            println!("Doing work {}", i);

            thread::sleep(Duration::from_secs(1));

            println!("Completed {}", i);
        });
        handles.push(handle);
    }

    manager.wait_all(handles);
}
