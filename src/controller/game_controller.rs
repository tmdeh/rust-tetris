
use std::thread;
use std::time::Duration;

use crate::controller::Display;
use crate::controller::KeyInput;

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
            display: Display::new(),
            keyInput: KeyInput::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            self.display.draw().unwrap();
            thread::sleep(Duration::from_millis(100))
        }
    }

}

