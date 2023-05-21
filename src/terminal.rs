use termion::color;



const MAX_X: u32 = 12;
const MAX_Y: u32 = 21;

pub struct Display {
    map: Vec<Vec<i32>>,
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
        }
    }

    pub fn draw(&mut self) {
        clearscreen::clear().unwrap();

        for y in 0..self.map.len() {
            let length = self.map[y].len();
            for x in 0..length {
                if self.map[y][x] == 1 {
                    print!("{}██", color::Fg(color::White))
                } else {
                    print!("  ");
                }
            }
            println!("");
        }
    }


}
