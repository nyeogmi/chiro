use chiro::{Window, Font, Drawable, Eventable};

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        (80, 60), 
        0x000000,
        0xc0c0c0,
    );

    win.at_i((1, 1)).put("IT'S A BAT PARTY!! WHOA!");
    win.at_i((1, 3)).font(Font::Small).fg((128, 0, 0)).put("IT'S A BAT PARTY!! WHOA!");

    loop {
        let evt = win.next_char();
        if let Some(te) = evt {
            println!("{:?}", te);
        } else {
            return
        }
    }
}