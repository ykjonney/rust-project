// #![feature(once_cell)]
use std::{sync::OnceLock,thread};

fn main() {
    // let b = Box::new(10);
    // let prt = Box::into_raw(b);
    // let x= unsafe {
    //     Box::from_raw(prt)
    // };
    let handle = thread::spawn(|| {
        let logger = Logger::global();
        logger.log("thread message".to_string());
    });
    let logger = Logger::global();
    logger.log("main thread message".to_string());
    handle.join().unwrap();
    
}

#[derive(Debug)]
struct Logger;
static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    fn global() -> &'static Logger {
        LOGGER.get_or_init(|| {
            println!("logger is being created...");
            Logger
        })
    }
    fn log(&self, message: String) {
        println!("{message}");
    }
}
