use std::{
    io::{stdout, Stdout},
    process::exit,
    thread,
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use tui::{backend::CrosstermBackend, Terminal};

pub struct Display {
    stdout: Stdout,
    // backend: CrosstermBackend<Stdout>,
    terminal: Terminal<CrosstermBackend<Stdout>>, 
    width: i32,
    height: i32
}

impl Display {
    pub fn new(w: i32, h: i32) -> Self {
        enable_raw_mode().unwrap();
        thread::spawn(Display::key_event);

        let stdout = stdout();
        let backend =  CrosstermBackend::new(&stdout);
        let terminal = Terminal::new(backend).unwrap();
        

        Display {
            stdout,
            backend,
            terminal,
            width: w,
            height: h,
        }
    }

    pub fn key_event() -> crossterm::Result<()> {
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

    fn draw(&self) {
        
    }
}
