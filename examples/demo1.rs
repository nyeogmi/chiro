use chiro::{Window, Color, Font, Drawable};
use euclid::size2;

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        size2(80, 60), 
        Color::rgb(0, 0, 0),
        Color::rgb(192, 192, 192),
    );

    win.at_i((1, 1)).puts("IT'S A BAT PARTY!! WHOA!");
    win.at_i((1, 3)).font(Font::Small).push_mod(&|z| z.fg = Color::rgb(128, 0, 0)).puts("IT'S A BAT PARTY!! WHOA!");

    win.getch();
}