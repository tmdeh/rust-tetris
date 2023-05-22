

mod terminal;

use std::{thread, time::Duration};

use terminal::Display;



fn main() { 
    let mut display = Display::new();


    thread::scope(|_| {
        loop {
            display.draw();
            thread::sleep(Duration::from_millis(500));
        }
    })

    
}