use std::{thread, time::Duration};


mod controller;

const W: u16 = 10;
const H: u16 = 20;
const M: u16 = 2;

fn main() {
    let mut display = controller::Display::new(W * M, H * M);   
    display.draw().unwrap();
    let _key_input = controller::KeyInput::new();
    loop {
        
        thread::sleep(Duration::from_millis(100))
    }
}
