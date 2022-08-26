use chiro::{Window, Drawable, Eventable, FString, Font};

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        (80, 60), 
        0x000000,
        0xc0c0c0,
    );

    win.at_i((1, 1)).puts(
        FString::from("Let me see your ") + 
        FString::from("bats").bg(0xffff00).fg(0x000000) + 
        FString::from(".")
    ).fg(0x606060).font(Font::Small).shift((0, 1)).puts(" (heh heh heh...)");

    loop {
        let evt = win.next_char();
        if let Some(te) = evt {
            println!("{:?}", te);
        } else {
            return
        }
    }
}