
use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use chiro::{Window, Color, Drawable, Eventable};
use euclid::size2;

fn main() {
    let mut win = Window::new(
        "bat party 2".to_string(), 
        size2(80, 60), 
        Color::rgb(0, 0, 0),
        Color::rgb(192, 192, 192),
    );

    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+-=[]{}|\\;:'\",./<>?                           ".chars().collect();

    for i in 0.. {
        for x in 0..80 {
            for y in 0..30 {
                let y2 = y * 2;

                let mut h = DefaultHasher::new();
                (i, x, y, 0).hash(&mut h);
                let hsh = h.finish();

                let mut h = DefaultHasher::new();
                (i, x, y, 1).hash(&mut h);
                let hsh_rgb = h.finish();

                let fg = Color::rgb((hsh_rgb & 0xff) as u8, (hsh_rgb >> 8 & 0xff) as u8, (hsh_rgb >> 16 & 0xff) as u8);
                let bg = Color::rgb((hsh_rgb >> 24 & 0xff) as u8, (hsh_rgb >> 32 & 0xff) as u8, (hsh_rgb >> 40 & 0xff) as u8);

                win.at_i((x, y2)).push_mod(&|z| { z.fg = fg; z.bg = bg; } ).putc(chars[hsh as usize % chars.len()]);
            }
        }

        let evt = win.next_tick();
        if let None = evt {
            return
        }
    }
}