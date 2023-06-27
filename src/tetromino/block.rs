use rand::{self, seq::SliceRandom, thread_rng, Rng};


#[derive(Debug)]
enum BlockType {
    I,
    O,
    T,
    L,
    J,
    S,
    Z
}

pub struct Block {
    b_type: BlockType
}

impl Block {
    pub fn new() -> Self {
        let rng = &mut thread_rng();

        let nums = vec![0,1,2, 3,4,5,6];
    
        let b = match nums.choose(rng).unwrap() {
            0 => BlockType::I,
            1 => BlockType::O,
            2 => BlockType::T,
            3 => BlockType::L,
            4 => BlockType::J,
            5 => BlockType::S,
            6 => BlockType::Z,
            _ => panic!("블록 생성 오류")
        };



        Block {
            b_type: b,
        }
    }
}