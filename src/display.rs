use std::io::{stdout, Stdout};
use crossterm::{execute, terminal, style::{self, Stylize}, QueueableCommand, cursor, queue};


const X_MAX: i32 = 100;
const Y_MAX: i32 = 200;

pub struct Window {
    map: Vec<Vec<u16>>
}

impl Window{
    pub fn new() -> Self {
        let mut yv: Vec<Vec<u16>> = Vec::new();
        
        for y in 0..=Y_MAX + 1 {
            let mut xv: Vec<u16> = Vec::new();
            for x in 0..=X_MAX + 2 {
                if x == 0 || x == X_MAX + 2 || y == Y_MAX + 1 {
                    xv.push(1);
                }
                else {
                    xv.push(0);
                }
            }
            yv.push(xv);
        }

        Window {
            map : yv
        }
    }

    pub fn draw(&self, stdout: &mut Stdout) {
        execute!(stdout, terminal::Clear(terminal::ClearType::All));

        for y in 0..self.map.len() {
            for x in 0 ..self.map[y].len() {
                if self.map[y][x] == 1 {
                    queue!(stdout, cursor::MoveTo(x as u16,y as u16), style::PrintStyledContent( "█".magenta()));
                }
            }
        }

    }
}