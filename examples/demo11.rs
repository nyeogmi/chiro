use std::process::exit;

use chiro::*;
use chiro::minifb::Window;
use chiro::simple_io::*;

fn main() {
    let mut win = Window::new(
        "bat party".to_string(), 
        (80, 60), 
        0x000000,
        0xc0c0c0,
        Box::new(|| exit(0)),
    );


    let mut i: u32 = 0;
    loop {
        i = i.wrapping_add(1);

        let mut data: [[u32; 8]; 8] = [[0; 8]; 8];
        for y in 0..8 {
            for x in 0..8 {
                let color_x = x as u32 * 8;
                let color_y = y as u32 * 8;
                data[x][y] = (color_x << 16) as u32 | (color_y << 8) as u32 | i;
            }
        }

        let tile = SuperTile::new(data);

        for cy in 0..60 {
            for cx in 0..80 {
                win.at_i((cx, cy)).put_st(tile.clone());
            }
        }

        let _ = win.tick();
        if win.input().keyboard().any_is_pressed() { break; }
    }

    win.draw().clear();
    win.at((0, 2)).put(" A B ");
    println!("{:?}", " A B ".to_fstring());
    win.at((0, 0)).put("All done! No more art.");

    loop {
        let _ = win.tick();
        if win.input().keyboard().any_is_pressed() { break; }
    }
}