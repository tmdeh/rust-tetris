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
#[derive(Debug)]
pub struct Block {
    b_type: BlockType,
    b_value: Vec<Vec<u8>>
}

impl Block {
    pub fn new() -> Self {
        let rng = &mut thread_rng();

        let nums = vec![0,1,2, 3,4,5,6];
    
        let t = match nums.choose(rng).unwrap() {
            0 => BlockType::I,
            1 => BlockType::O,
            2 => BlockType::T,
            3 => BlockType::L,
            4 => BlockType::J,
            5 => BlockType::S,
            6 => BlockType::Z,
            _ => panic!("블록 생성 오류")
        };

        let b_value = Block::make_block(&t);

        Block {
            b_type: t,
            b_value
        }
    }

    fn make_block(t: &BlockType) -> Vec<Vec<u8>> {
        // 4 * 2    
        match t {
            BlockType::I => {
                return vec![
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![1, 1, 1, 1]
                ];
            },
            BlockType::O => {
                return vec![
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![2, 2, 0, 0],
                    vec![2, 2, 0, 0]
                ];
            },
            BlockType::T => {
                return vec![
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0,3,0,0],
                    vec![3,3,3,0]
                ];
            }
            BlockType::L => {
                return vec![
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 4, 0],
                    vec![4, 4, 4, 0]
                ];
            },
            BlockType::J => {
                return vec![
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![5, 0, 0, 0],
                    vec![5, 5, 5, 0]
                ]
            },
            BlockType::S => {
                return vec![
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 6, 6, 0],
                    vec![6, 6, 0, 0]
                ]
            },
            BlockType::Z => {
                return vec![
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![7, 7, 0, 0],
                    vec![0, 7, 7, 0]
                ]
            }
            _ => panic!("블록 타입 오류")
        }
    }

}