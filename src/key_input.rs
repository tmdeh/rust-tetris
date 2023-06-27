use std::{thread, time::Duration, process::exit};

use crossterm::{event::{poll, Event, KeyCode, KeyModifiers, read}, terminal::disable_raw_mode};


pub struct KeyInput {}

impl KeyInput {
    pub fn new() -> Self {
        // 키 이벤트 쓰레드 생성

        thread::spawn(KeyInput::key_event);
        KeyInput { }
    }

    fn key_event() -> crossterm::Result<()> {
        loop {
            if poll(Duration::from_millis(500))? {
                match read()? {
                    Event::Key(event) => {
                        match (event.code, event.modifiers) {
                            (KeyCode::Up, _) => {
                                println!("Up")
                            }
                            (KeyCode::Down, _) => {
                                println!("Down")
                            }
                            (KeyCode::Left, _) => {
                                println!("Left")
                            }
                            (KeyCode::Right, _) => {
                                println!("Right")
                            }
                            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                                disable_raw_mode().unwrap();
                                exit(0x0100);
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