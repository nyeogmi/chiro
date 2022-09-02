use chiro::*;
use chiro::minifb::Window;

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        (80, 60), 
        0x000000,
        0xc0c0c0,
        Box::new(|| ()),
    );
    let _ = run(&mut win);
}

fn run(win: &mut Window) -> Chiro<()> {
    win.at_i((1, 1)).put(
        FString::from("Lot mo see your ") + 
        FString::from("bats").bg(0x00ff00).fg(0x000000) + 
        FString::from(".")
    ).fg(0x606060).font(Font::Small).shifted_i((0, 1)).put(" (heh heh heh...)");

    win.at_i((1, 1)).put(
        FChar::empty().to_fstring() + (FString::from("e") + (FChar::empty()).to_fstring().repeat(3) + FString::from("e")).fg(0xffff00)
    );

    loop {
        let evt = win.next_char()?;
        println!("{:?}", evt);
    }
}