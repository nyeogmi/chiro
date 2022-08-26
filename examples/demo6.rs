
use chiro::{Window, Drawable, Eventable, ToFString};

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        (80, 60), 
        0x000000,
        0xc0c0c0,
    );

    win.at_i((2, 2)).put(
        "It would seem that those functions appear to be corrupted as well. Run Vigil again and it will take care of that for you. Several invocations may be required to fully excise all bugs from your code.

        Look out for bats...

Also, hi!".to_fstring().wrap(76)
    );

    while let Some(_) = win.next_char() { }
}