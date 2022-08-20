use chiro::{Window, Eventable, Drawable};

fn main() {
    let mut win = Window::new(
        "bat party 3".to_string(), 
        (80, 60), 
        (0, 0, 0),
        (192, 192, 192),
    );

    let hello = win.affordance();
    let goodbye = win.affordance();

    win.at_i((2, 2)).affordance(hello).puts("hello!!");
    win.at_i((2, 4)).affordance(goodbye).puts("goodbye!!");

    loop {
        let evt = win.next_tick();
        if let None = evt {
            return
        }

        let mouse = win.input().mouse();
        if mouse.left_clicked(hello) { println!("left-clicked hello!"); }
        if mouse.right_clicked(hello) { println!("right-clicked hello!"); }
        if mouse.left_clicked(goodbye) { println!("left-clicked goodbye!"); }
        if mouse.right_clicked(goodbye) { println!("right-clicked goodbye!"); }
    }
}