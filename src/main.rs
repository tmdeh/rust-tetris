
mod display;
mod key_input;

fn main() {
    display::Display::new(100, 300);   
    key_input::KeyInput::new();
    loop {}
}
