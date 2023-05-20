const X_MAX: i32 = 10;
const Y_MAX: i32 = 20;

pub struct Window {
    map: Vec<Vec<i32>>
}

impl Window{
    pub fn new() -> Self {
        let mut yv: Vec<Vec<i32>> = Vec::new();
        
        for y in 0..=Y_MAX + 1 {
            let mut xv: Vec<i32> = Vec::new();
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
}

