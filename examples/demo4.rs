use chiro::*;
use chiro::minifb::Window;

fn main() {
    let mut win = Window::new(
        "bat party 4".to_string(),
        (80, 60), 
        (0, 0, 0),
        (192, 192, 192),
        Box::new(|| ()),
    );

    let _ = run(&mut win);
}

fn run(win: &mut Window) -> Chiro<()> {
    win.at_i((2, 2)).fill_rect_i((78, 58), 'a');
    win.at_i((4, 4)).bg(0xff0000).fg(0).touch_rect_i((10, 9));
    win.at_i((4, 9)).bg(0xffff00).fg(0).touch_rect_i((10, 10));
    win.at_i((5, 5)).bg(0x000080).fg(0xc0c080).fill_rect_i((12, 12), 'B');

    win.clip_i((1, 1), (5, 5)).fg(0xff0000).fill('Z');

    loop {
        let _ = win.next_tick()?;
    }
}