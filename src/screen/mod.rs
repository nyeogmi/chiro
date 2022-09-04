use std::collections::HashMap;

use euclid::*;

use crate::{shared::*, tileset::Tile, Font};

mod affordances;
mod dirtyregion;
mod render;
mod pixelfb;

pub use dirtyregion::DirtyRegion;
pub use pixelfb::PixelFB;

use self::affordances::Affordances;

pub struct Screen {
    pub(crate) size: ZelSize,
    pub(crate) affordances: Affordances,
    pub(crate) zels: Vec<ZelData>,
    pub(crate) zel_supertiles: HashMap<Zel, SuperTile>,
    pub(crate) bg: Color, pub(crate) fg: Color,
}

impl Screen {
    pub fn new(size: impl ToZelSize, bg: impl ToColor, fg: impl ToColor) -> Self {
        let zels = vec![];
        let mut screen = Self { 
            size: size2(0, 0), 
            affordances: Affordances::new(), 
            zels, zel_supertiles: HashMap::new(),
            bg: bg.to_color(), fg: fg.to_color() 
        };
        screen.resize(size);
        screen
    }

    pub fn resize(&mut self, size: impl ToZelSize) {
        let size = size.to_zels();
        self.size = size;
        self.zels = vec![ZelData::default(); (size.width * size.height) as usize];
    }

    pub fn raw_view(&self, xy: Zel) -> (ZelData, Option<&SuperTile>) {
        if xy.x < 0 || xy.y < 0 || xy.x as u32 >= self.size.width || xy.y as u32 >= self.size.height {
            return (ZelData::default(), None)
        }
        return (self.zels[(xy.y as u32 * self.size.width + xy.x as u32) as usize], self.zel_supertiles.get(&xy))
    }
}

impl Clone for Screen {
    fn clone(&self) -> Self {
        Self { 
            size: self.size.clone(), 
            affordances: self.affordances.clone(), 
            zels: self.zels.clone(), 
            zel_supertiles: self.zel_supertiles.clone(), 
            bg: self.bg.clone(), fg: self.fg.clone() 
        }
    }

    fn clone_from(&mut self, other: &Screen) {
        self.size = other.size.clone();
        self.affordances = other.affordances.clone();
        self.zels.clone_from(&other.zels);
        self.bg = other.bg;
        self.fg = other.fg;
    }
}

#[derive(Clone, Copy, Default)]
pub struct ZelData {
    pub tile: Tile,
    pub click: Option<Affordance>,
    pub scroll: Option<Affordance>,

    pub bg: Color, pub fg: Color, 
}

impl Drawable for Screen {
    fn affordance(&mut self) -> Affordance {
        self.affordances.generate()
    }

    fn get_font(&self) -> crate::Font {
        return Font::Normal
    }

    fn bounds(&self) -> ZelRect {
        Rect::new(point2(0, 0), size2(self.size.width as i32, self.size.height as i32))
    }

    fn raw_touch(&mut self, xy: Zel, _format: bool, cb: impl FnOnce(&mut ZelData)) {
        if xy.x < 0 || xy.y < 0 || xy.x as u32 >= self.size.width || xy.y as u32 >= self.size.height {
            return 
        }
        cb(self.zels.get_mut((xy.y as u32 * self.size.width + xy.x as u32) as usize).expect("if zels is correctly formed, this zel exists"));
        self.zel_supertiles.remove(&xy);
    }

    fn deposit_supertile(&mut self, xy: Zel, tile: SuperTile) {
        if xy.x < 0 || xy.y < 0 || xy.x as u32 >= self.size.width || xy.y as u32 >= self.size.height {
            return 
        }
        self.zels[(xy.y as u32 * self.size.width + xy.x as u32) as usize] = ZelData::default();
        self.zel_supertiles.insert(xy, tile);
    }
}