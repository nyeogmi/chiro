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

    let para1 = "It would seem that those functions appear to be corrupted as well. Run Vigil again and it will take care of that for you. Several invocations may be required to fully excise all bugs from your code.".to_fstring().fg(0xffff00);
    let para2 = "
 Look out for bats...  
    AAAAAAAAAAAAAAAAAAAAAA

Also, hi!".to_fstring().bg(0x00ff00).fg(0x000000);
   

    win.at_i((2, 2)).put((para1 + "\n".to_fstring() + para2).wrap(76));

    loop {
        let _ = win.char();
    }
}