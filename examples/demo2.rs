
use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, process::exit};

use chiro::*;
use chiro::simple_io::*;
use chiro::minifb::Window;

fn main() {
    let mut win = Window::new(
        "bat party 2".to_string(), 
        (80, 60), 
        (0, 0, 0),
        (192, 192, 192),
        Box::new(|| exit(0)),
    );

    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+-=[]{}|\\;:'\",./<>?                           ".chars().collect();

    for i in 0.. {
        let _ = win.tick();
        for x in 0..80 {
            for y in 0..30 {
                let y2 = y * 2;

                let mut h = DefaultHasher::new();
                (i, x, y, 0).hash(&mut h);
                let hsh = h.finish();

                let mut h = DefaultHasher::new();
                (i, x, y, 1).hash(&mut h);
                let hsh_rgb = h.finish();

                win.at_i((x, y2))
                    .fg((hsh_rgb & 0xffffff) as u32)
                    .bg(((hsh_rgb >> 24) & 0xffffff) as u32)
                    .put(chars[hsh as usize % chars.len()]);
            }
        }
    };
}