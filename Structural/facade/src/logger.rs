pub trait Logger: Send + Sync {
    fn info(&self, msg: &str);
}

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn info(&self, msg: &str) {
        println!("[INFO] {}", msg);
    }
}

pub struct FileLogger;

impl Logger for FileLogger {
    fn info(&self, msg: &str) {
        println!("[FILE] {}", msg);
    }
}
