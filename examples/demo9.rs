use chiro::*;
use chiro::minifb::Window;

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        (80, 60), 
        0x000000,
        0xc0c0c0,
        Box::new(|| println!("closed!")),
    );
    let _ = run(&mut win);
}

fn run(win: &mut Window) -> Chiro<()> {
    loop {
        let _ = win.next_tick()?;
        println!("open?: {}", win.is_open())
    }
}