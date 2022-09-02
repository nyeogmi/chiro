use std::process::exit;

use chiro::*;
use chiro::minifb::Window;

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        (80, 60), 
        0x000000,
        0xc0c0c0,
        Box::new(|| exit(0)),
    );

    let _ = run(&mut win);
}

fn run(win: &mut Window) -> Chiro<()> {
    win.at_i((1, 1)).put("IT'S A BAT PARTY!! WHOA!");
    win.at_i((1, 3)).font(Font::Small).fg((128, 0, 0)).put("IT'S A BAT PARTY!! WHOA!");

    loop {
        println!("{:?}", win.next_char()?);
    }
}