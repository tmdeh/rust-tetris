use std::{io::{stdout, Write, Stdout}};

use crossterm::{QueueableCommand, cursor, ExecutableCommand, terminal, style::{ self } };

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
        
        let mut x_delta: u16 = 0;

        for (y, line) in self.map.iter().enumerate() {
          for (x, v) in line.iter().enumerate() {
            if *v == 9 {
                print_block(&mut self.stdout, x as u16, y as u16,"\x1b[42m", &mut x_delta).unwrap();
            }
            else {
                print_block(&mut self.stdout, x as u16, y as u16,"\x1b[40m", &mut x_delta).unwrap();
            }
          }
          x_delta = 0;
        }
        self.stdout.flush()?;
        Ok(())
    }
}

fn print_block(stdout: &mut Stdout, x: u16, y: u16, s :&str, x_delta: &mut u16) -> std::io::Result<()> {
    stdout
        .queue(style::Print(s))?
        .queue(cursor::MoveTo(x + *x_delta, y))?;
    *x_delta += 1;
    stdout
        .queue(style::Print("  "))?
        .queue(cursor::MoveTo(x + *x_delta, y))?
        .queue(style::Print("\x1b[0m"))?
        .queue(cursor::MoveTo(x + *x_delta, y))?;

    Ok(())
}
