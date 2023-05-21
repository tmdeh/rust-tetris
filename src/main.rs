use std::thread;
use std::time::Duration;
use crossterm::event::{poll, read, Event};

mod display;

use display::Window;

fn main() {
    let window: Window = Window::new();

    // TODO: 0.5초 마다 다시 재출력 기능

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
