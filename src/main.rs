

mod terminal;

use std::{thread, time::Duration};

use crossterm::event::{read, Event, KeyCode, poll};
use terminal::Display;



fn main() { 
    let mut display = Display::new();

    thread::spawn(|| {
        loop {
            if poll(Duration::from_millis(500)).unwrap() {
                match read().unwrap() {
                    Event::Key(event) => {
                        match event.code {
                            KeyCode::Backspace => println!("Back"),
                            KeyCode::Up => println!("Up"),
                            KeyCode::Down => println!("Down"),
                            KeyCode::Left => println!("Left"),
                            KeyCode::Right => println!("Right"),
                            _ => println!("None")
                        };
                    }
                    _ => {}
                }
            }
        }
    });

    thread::scope(|_| {
        loop {
            display.draw();
            thread::sleep(Duration::from_millis(200));
        }
    });

}