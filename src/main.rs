use std::{thread, time::Duration, collections::btree_map::Range};

use tetromino::Block;


mod controller;
mod tetromino;

const W: u16 = 10;
const H: u16 = 20;
const M: u16 = 1;

fn main() {
    let mut display = controller::Display::new(W * M, H * M);   
    display.draw().unwrap();
    let _key_input = controller::KeyInput::new();
    
    let block = Block::new();

    loop {

        thread::sleep(Duration::from_millis(100))
    }
}
