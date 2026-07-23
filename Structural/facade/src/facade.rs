use crate::logger::Logger;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct ThreadManager {
    logger: Arc<Mutex<dyn Logger>>,
}

impl ThreadManager {
    pub fn new(logger: Arc<Mutex<dyn Logger>>) -> Self {
        Self { logger }
    }

    pub fn spawn<F>(&self, name: String, job: F) -> JoinHandle<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let logger = Arc::clone(&self.logger);

        logger
            .lock()
            .unwrap()
            .info(&format!("Creating thread {}", name));

        thread::Builder::new()
            .name(name)
            .spawn(move || {
                logger.lock().unwrap().info(&format!(
                    "Started {}",
                    thread::current().name().unwrap_or("Unnamed thread")
                ));

                job();

                logger.lock().unwrap().info(&format!(
                    "Finished {}",
                    thread::current().name().unwrap_or("Unnamed thread")
                ));
            })
            .expect("Failed to spawn thread")
    }

    pub fn wait_all(&self, handles: Vec<JoinHandle<()>>) {
        for handle in handles {
            if handle.join().is_err() {
                self.logger.lock().unwrap().info("Thread panicked");
            }
        }
    }
}
