use chiro::{Window, Drawable, Eventable, FString, Font, FChar, ToFString};

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        (80, 60), 
        0x000000,
        0xc0c0c0,
    );

    win.at_i((1, 1)).puts(
        FString::from("Lot mo see your ") + 
        FString::from("bats").bg(0x00ff00).fg(0x000000) + 
        FString::from(".")
    ).fg(0x606060).font(Font::Small).shift_i((0, 1)).puts(" (heh heh heh...)");

    win.at_i((1, 1)).puts(
        FChar::empty().to_fstring() + (FString::from("e") + (FChar::empty()).to_fstring().repeat(3) + FString::from("e")).fg(0xffff00)
    );

    loop {
        let evt = win.next_char();
        if let Some(te) = evt {
            println!("{:?}", te);
        } else {
            return
        }
    }
}