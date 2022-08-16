use chiro::{Window, Color, Font};
use euclid::size2;

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        size2(80, 60), 
        Color::rgb(0, 0, 0),
        Color::rgb(192, 192, 192),
    );

    win.screen().puts(1, 1, Font::Normal, "IT'S A BAT PARTY!! WHOA!");
    win.screen().puts(1, 3, Font::Small, "IT'S A BAT PARTY!! WHOA!");

    win.getch();
}