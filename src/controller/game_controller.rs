
use crate::controller::Display;
use crate::controller::KeyInput;

const W: u16 = 10;
const H: u16 = 20;
const M: u16 = 1;

// 게임 흐름 제어 객체
enum GameState {
    PROGRESS, // 블럭이 내려가는 중
    END, // 게임 오버
}

pub struct GameController {
    // 게임 상태 관리
    state: GameState,
    display: Display,
    keyInput: KeyInput
}


impl GameController {
    pub fn new() -> Self {
        GameController { 
            state: GameState::PROGRESS,
            display: Display::new(W * M, H * M),
            keyInput: KeyInput::new()
        }
    }

    pub fn run(&mut self) {

        self.display.draw().unwrap();

        loop {
        
        }
    }

}

