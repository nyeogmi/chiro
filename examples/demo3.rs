use chiro::{Window, Eventable, Drawable};

fn main() {
    let mut win = Window::new(
        "bat party 3".to_string(), 
        (80, 60), 
        (0, 0, 0),
        (192, 192, 192),
    );

    let aff = win.affordance();

    loop {
        win.at_i((2, 2)).affordance(aff).puts("hello!!");

        let evt = win.next_tick();
        if let None = evt {
            return
        }

        let mouse = win.input().mouse();
        if mouse.left_clicked(aff) {
            println!("left-clicked hello!");
        }
        if mouse.right_clicked(aff) {
            println!("right-clicked hello!");
        }
    }
}