use std::thread;
use std::time::Duration;
use crossterm::event::{poll, read, Event};

mod display;

use display::Window;

fn main() {

    let window: Window = Window::new();

    // 입력 스레드
    thread::spawn(||{
        loop {
            if poll(Duration::from_millis(500)).unwrap() {
                match read().unwrap() {
                    Event::Key(event) => println!("{:?}", event),
                    _ => println!("Nope"),
                }
            }
        }
    });
}
