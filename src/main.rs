use std::time::SystemTime;
use std::{thread, time}; 
use std::io;
use std::io::Write;

fn main() {
    println!("Hello, world!");




    for i in 0..10_000 {
        let start_time = SystemTime::now();

        // start
        thread::sleep(time::Duration::from_millis(10));
        // end

        let end_time = SystemTime::now();
        end_time.duration_since(start_time).unwrap();

        if i % 100 == 0 {
            print!(".");
            io::stdout().flush();
        }
    }
}
