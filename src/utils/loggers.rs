pub trait Logger {
    fn log(&mut self, message: String);
}

pub struct FnLogger<F>(F);

impl <F> Logger for FnLogger<F>
where F: FnMut(String)
{
    fn log(&mut self, message: String) {
        (self.0)(message);
    }
}

pub fn noop_logger() -> impl Logger {
    FnLogger(|_| {})
}

pub fn stdout_logger() -> impl Logger {
    FnLogger(|message| println!("{message}"))
}
