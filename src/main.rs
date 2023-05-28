use std::{time::Duration, thread, process::exit};

use crossterm::{event::{read, Event, poll, KeyCode, ModifierKeyCode, KeyModifiers}, terminal::enable_raw_mode};

fn main() { 

    enable_raw_mode();

    thread::spawn(key_event);

    loop {   
    }
}



fn key_event() -> crossterm::Result<()> {
    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Up => { println!("Up") },
                        KeyCode::Down => {println!("Down")},
                        KeyCode::Left => {println!("Left")},
                        KeyCode::Right => {println!("Right")}, 
                        KeyCode::Char('c') => {
                            if event.modifiers == KeyModifiers::CONTROL {
                                exit(0x0100);
                            }
                        },
                        _ => ()
                    };
                },
                _ => ()
            };
        }
    }
}
