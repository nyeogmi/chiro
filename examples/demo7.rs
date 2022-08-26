use chiro::{Window, Drawable, Eventable, BoxArt, Font};

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        (80, 60), 
        0x000000,
        0xc0c0c0,
    );

    let mut box_art = BoxArt::new(Font::Small);
    box_art.draw_box((-4, -4), (4, 4), true);
    box_art.draw(win.at((4, 4)));

    win.at((1, 1)).bg(0x770000).fill_rect((7, 7), 'a');

    let mut box_art = BoxArt::new(Font::Normal);
    box_art.draw_box((-4, -4), (4, 4), true);
    box_art.draw(win.at((14, 4)));

    win.at((11, 2)).bg(0x770000).fill_rect((17, 6), 'a');

    let mut box_art = BoxArt::new(Font::Fat);
    box_art.draw_box((-4, -4), (4, 4), true);
    box_art.draw(win.at((24, 4)));

    win.at((22, 2)).bg(0x770000).fill_rect((26, 6), 'a');

    let mut box_art = BoxArt::new(Font::Small);
    box_art.draw_box((-4, -4), (4, 4), false);
    box_art.draw(win.at((4, 14)));

    win.at((1, 11)).bg(0x770000).fill_rect((7, 17), 'a');

    let mut box_art = BoxArt::new(Font::Normal);
    box_art.draw_box((-4, -4), (4, 4), false);
    box_art.draw(win.at((14, 14)));

    win.at((11, 12)).bg(0x770000).fill_rect((17, 16), 'a');

    let mut box_art = BoxArt::new(Font::Fat);
    box_art.draw_box((-4, -4), (4, 4), false);
    box_art.draw(win.at((24, 14)));

    win.at((22, 12)).bg(0x770000).fill_rect((26, 16), 'a');

    let mut box_art = BoxArt::new(Font::Small);
    box_art.draw_box((0, 0), (8, 8), false);
    box_art.draw_box((0, 0), (8, 0), true);
    box_art.draw_box((0, 2), (8, 2), true);
    box_art.draw_box((3, 3), (9, 9), true);
    box_art.draw(win.at((40, 0)));


    while let Some(_) = win.next_char() { }
}