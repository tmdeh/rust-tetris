mod controller;

use controller::Display;

fn main() {
    Display::new(100, 300);
    loop {}
}
