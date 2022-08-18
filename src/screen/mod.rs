use euclid::*;

use crate::{aliases::*, tileset::Tile, color::Color};
mod dirtyregion;
mod render;
mod pixelfb;

pub use dirtyregion::DirtyRegion;
pub use pixelfb::PixelFB;

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
}

impl Clone for Screen {
    fn clone(&self) -> Self {
        Self { size: self.size.clone(), zels: self.zels.clone(), bg: self.bg.clone(), fg: self.fg.clone() }
    }

    fn clone_from(&mut self, other: &Screen) {
        self.size = other.size.clone();
        self.zels.clone_from(&other.zels);
    }
}

#[derive(Clone, Copy, Default)]
pub struct Zel {
    pub tile: Tile,
    pub affordance: Option<Affordance>,

    pub bg: Color, pub fg: Color, 
}

impl Drawable for Screen {
    fn raw_view(&self, xy: ZelPointI) -> Zel {
        if xy.x < 0 || xy.y < 0 || xy.x as u32 >= self.size.width || xy.y as u32 >= self.size.height {
            return Zel::default()
        }
        return self.zels[(xy.y as u32 * self.size.width + xy.x as u32) as usize]
    }

    fn raw_at(&mut self, xy: ZelPointI) -> Option<&mut Zel> {
        if xy.x < 0 || xy.y < 0 || xy.x as u32 >= self.size.width || xy.y as u32 >= self.size.height {
            return None
        }
        return self.zels.get_mut((xy.y as u32 * self.size.width + xy.x as u32) as usize)
    }

    fn clear(&mut self) {
        self.resize(self.size)
    }
}