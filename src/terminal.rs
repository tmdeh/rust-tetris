use std::io::{Stdout, stdout, Write};

use crossterm::{ style::{ Stylize, PrintStyledContent, Print }, execute, cursor, terminal, ExecutableCommand };



const MAX_X: u32 = 32;
const MAX_Y: u32 = 61;

pub struct Display {
    map: Vec<Vec<i32>>,
    stdout: Stdout
}

impl Display {
    pub fn new() -> Self{
        
        let mut m: Vec<Vec<i32>> = Vec::new();

        for y in 0..=MAX_Y {
            let mut xv: Vec<i32> = Vec::new();
            for x in 0..=MAX_X {
                if x == 0 || x == MAX_X || y == MAX_Y {
                    xv.push(1);
                } else {
                    xv.push(0);
                }
            }
            m.push(xv);
        }

        Display { 
            map: m,
            stdout: stdout()
        }
    }

    pub fn draw(&mut self) {

        self.stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();

        for y in 0..self.map.len() {
            let length = self.map[y].len();
            for x in 0..length {
                if self.map[y][x] == 1 {
                    execute!(self.stdout, cursor::MoveTo((x as u16) + 100, y as u16), PrintStyledContent( "██".white())).unwrap();
                }
            }
        }

        self.stdout.flush().unwrap();

    }


}
