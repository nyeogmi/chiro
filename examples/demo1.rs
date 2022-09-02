use std::process::exit;

use chiro::*;
use chiro::simple_io::*;
use chiro::minifb::Window;

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        (80, 60), 
        0x000000,
        0xc0c0c0,
        Box::new(|| exit(0)),
    );

    win.at_i((1, 1)).put("IT'S A BAT PARTY!! WHOA!");
    win.at_i((1, 3)).font(Font::Small).fg((128, 0, 0)).put("IT'S A BAT PARTY!! WHOA!");

    loop {
        println!("{:?}", win.char());
    }
}