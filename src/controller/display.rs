use std::{
    io::{stdout, Stdout},
};

use crossterm::{
    terminal::{enable_raw_mode},
};
use tui::{backend::CrosstermBackend, Terminal, layout::{Layout, Constraint, Direction}};

pub struct Display {
    terminal: Terminal<CrosstermBackend<Stdout>>, 
    width: i32,
    height: i32
}

impl Display {
    pub fn new(w: i32, h: i32) -> Self {
        enable_raw_mode().unwrap();

        let stdout = stdout();
        let backend =  CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        Display {
            terminal,
            width: w,
            height: h,
        }
    }

    fn draw(&self) {

    }
}
