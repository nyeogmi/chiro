use euclid::*;

use crate::{aliases::*, tileset::Tile, color::Color, Font};
mod render;
mod pixelfb;

pub use pixelfb::PixelFB;

#[derive(Clone)]
pub struct Screen {
    pub(crate) size: ZelSize,
    pub(crate) zels: Vec<Zel>,
    pub(crate) bg: Color, pub(crate) fg: Color,
}

impl Screen {
    pub fn new(size: ZelSize, bg: Color, fg: Color) -> Self {
        let zels = vec![];
        let mut screen = Self { size: size2(0, 0), zels, bg, fg };
        screen.resize(size);
        screen
    }

    pub fn resize(&mut self, size: ZelSize) {
        self.size = size;
        self.zels = vec![Zel::default(); (size.width * size.height) as usize];
    }

    pub fn clear(&mut self) {
        self.resize(self.size)
    }

    pub(crate) fn get(&self, x: u32, y: u32) -> Zel {
        if x >= self.size.width || y >= self.size.height {
            return Zel::default()
        }
        return self.zels[(y * self.size.width + x) as usize]
    }

    pub(crate) fn try_get_mut(&mut self, x: u32, y: u32) -> Option<&mut Zel> {
        if x >= self.size.width || y >= self.size.height {
            return None
        }
        return self.zels.get_mut((y * self.size.width + x) as usize)
    }

    pub fn puts(&mut self, x: u32, y: u32, font: Font, s: &str) {
        let w = font.char_size().width;
        for (i, c) in s.chars().enumerate() {
            font.char_to_tile(c, |local, tile| {
                if let Some(zel) = self.try_get_mut(x + w * (i as u32) + local.x, y + local.y) {
                    zel.tile = tile;
                }
            })
        }
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Zel {
    pub(crate) tile: Tile,
    pub(crate) affordance: Option<Affordance>,

    pub(crate) bg: Color, pub(crate) fg: Color, 
}