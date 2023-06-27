
mod controller;

fn main() {
    let display = controller::Display::new(100, 300);   
    let key_input = controller::KeyInput::new();
    loop {}
}
