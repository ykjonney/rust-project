
#![feature(once_cell)]
use std::{lazy::SyncOnceCell,thread};
fn main() {
    // let b = Box::new(10);
    // let prt = Box::into_raw(b);
    // let x= unsafe {
    //     Box::from_raw(prt)
    // }; 
    let handle = thread::spawn(||{
        let logger= Logger::global();
        logger.log("thread message".to_string());
    });
    let logger = Logger::global();
    logger.log("main thread message".to_string());
    handle.join().unwrap();
    let s =vec![104, 101, 108, 108, 111];
   
    
}

#[derive(Debug)]
struct Logger;
static LOGGER:SyncOnceCell<Logger> = SyncOnceCell::new();

impl Logger {
    fn global() -> &'static Logger{
        LOGGER.get_or_init(||{
            println!("logger is being created...");
            Logger
        })
    }
    fn log(&self,message:String){
        println!("{message}");
    }
}
