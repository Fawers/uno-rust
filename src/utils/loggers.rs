pub trait Logger {
    fn log(&mut self, message: String);
}

pub fn noop_logger() -> impl Logger {
    NoopLogger
}

pub fn stdout_logger() -> impl Logger {
    StdoutLogger
}

struct NoopLogger;

impl Logger for NoopLogger {
    fn log(&mut self, _: String) {}
}

struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&mut self, message: String) {
        println!("{message}");
    }
}
