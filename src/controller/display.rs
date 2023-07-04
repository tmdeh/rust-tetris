use std::{
    io::{stdout, Stdout, self}
};

use crossterm::{
    terminal::{enable_raw_mode},
};
use tui::{backend::CrosstermBackend, Terminal, layout::{ Rect }, widgets::{Block, Borders}, style::Style};

pub struct Display {
    terminal:Terminal<CrosstermBackend<Stdout>>,
    map: Vec<Vec<u8>>,
    width: u16,
    height: u16
}

impl Display {
    pub fn new(w: u16, h: u16) -> Self {
        let stdout = stdout();
        let backend =  CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("터미널 오류 발생");

        enable_raw_mode().unwrap();
        terminal.hide_cursor().unwrap();
        terminal.clear().unwrap();


        let frame = terminal.get_frame();

        let size = frame.size();
        if size.width < w && size.height < h {
            panic!("터미널 크기가 충분하지 않습니다.");
        }

        // 맵 데이터
        let mut field = vec![];
    
        for i in 0..11 {
            if i == 10 {
                field.push(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
                break;    
            } 
            field.push(vec![9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9]);
        }
        
        Display {
            terminal,
            width: w,
            height: h,
            map: field
        }
    }



    pub fn draw(& mut self) -> Result<(), io::Error> {

        self.terminal.draw(|f| {
            let size = Rect::new(0, 0, self.width, self.height);
            let block = Block::default()
            .title("Tetris")
            .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;
        Ok(())
    }
}
