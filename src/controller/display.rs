use std::{io::{stdout, Write, Stdout}};

use crossterm::{QueueableCommand, cursor, style::{ self }, ExecutableCommand, terminal};

pub struct Display {
    map: Vec<Vec<u8>>,
    stdout: Stdout
}

impl Display {
    pub fn new() -> Self {


        // 맵 데이터
        let mut field = vec![];
    
        for i in 0..21 {
            if i == 20 {
                field.push(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
                break;    
            } 
            field.push(vec![9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9]);
        }
        let mut stdout = stdout();

        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.execute(cursor::DisableBlinking).unwrap();

        Display {
            map: field,
            stdout: stdout
        }
    }



    pub fn draw(& mut self) -> std::io::Result<()> {
        for (y, line) in self.map.iter().enumerate() {
          for (x, v) in line.iter().enumerate() {
            self.stdout.queue(cursor::MoveTo(x as u16,y as u16))?;
            if *v == 9 {
              self.stdout
                    .queue(style::Print( "\x1b[42m  \x1b[0m"))?;
            } else {
                self.stdout
                    .queue(style::Print( "\x1b[40m  \x1b[0m"))?;
            }
          }
        }
        self.stdout.flush()?;
        Ok(())
    }
}
