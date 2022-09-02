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

    win.at((0, 0)).put("A");
    win.at((1, 0)).fg(0xff0000).put("B");
    win.at((2, 0)).fg(0xff0000).fg(0x00ff00).put("C");
    win.at((0, 2)).put("D");
    win.at((1, 2)).bg(0xff0000).put("E");
    win.at((2, 2)).bg(0xff0000).bg(0x00ff00).put("F");

    loop {
        let _ = win.tick();
    }
}