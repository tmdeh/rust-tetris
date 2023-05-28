use std::{
    io::{stdout, Stdout},
    process::exit,
    thread,
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
    terminal::enable_raw_mode,
};

pub struct Display {
    stdout: Stdout,
    width: i32,
    height: i32,
}

impl Display {
    pub fn new(w: i32, h: i32) -> Self {
        enable_raw_mode().unwrap();
        thread::spawn(Display::key_event);

        Display {
            stdout: stdout(),
            width: w,
            height: h,
        }
    }

    pub fn key_event() -> crossterm::Result<()> {
        loop {
            if poll(Duration::from_millis(500))? {
                match read()? {
                    Event::Key(event) => {
                        match event.code {
                            KeyCode::Up => {
                                println!("Up")
                            }
                            KeyCode::Down => {
                                println!("Down")
                            }
                            KeyCode::Left => {
                                println!("Left")
                            }
                            KeyCode::Right => {
                                println!("Right")
                            }
                            KeyCode::Char('c') => {
                                if event.modifiers == KeyModifiers::CONTROL {
                                    exit(0x0100);
                                }
                            }
                            _ => (),
                        };
                    }
                    _ => (),
                };
            }
        }
    }
}
