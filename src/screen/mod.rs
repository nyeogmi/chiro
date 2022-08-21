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
    pub(crate) zels: Vec<Zel>,
    pub(crate) bg: Color, pub(crate) fg: Color,
}

impl Screen {
    pub fn new(size: impl ToZelSize, bg: impl ToColor, fg: impl ToColor) -> Self {
        let zels = vec![];
        let mut screen = Self { size: size2(0, 0), affordances: Affordances::new(), zels, bg: bg.to_color(), fg: fg.to_color() };
        screen.resize(size);
        screen
    }

    pub fn resize(&mut self, size: impl ToZelSize) {
        let size = size.to_zels();
        self.size = size;
        self.zels = vec![Zel::default(); (size.width * size.height) as usize];
    }
}

impl Clone for Screen {
    fn clone(&self) -> Self {
        Self { size: self.size.clone(), affordances: self.affordances.clone(), zels: self.zels.clone(), bg: self.bg.clone(), fg: self.fg.clone() }
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
pub struct Zel {
    pub tile: Tile,
    pub click: Option<Affordance>,
    pub scroll: Option<Affordance>,

    pub bg: Color, pub fg: Color, 
}

impl Drawable for Screen {
    fn affordance(&mut self) -> Affordance {
        self.affordances.generate()
    }

    fn raw_view(&self, xy: ZelPointI) -> Zel {
        if xy.x < 0 || xy.y < 0 || xy.x as u32 >= self.size.width || xy.y as u32 >= self.size.height {
            return Zel::default()
        }
        return self.zels[(xy.y as u32 * self.size.width + xy.x as u32) as usize]
    }

    fn raw_touch(&mut self, xy: ZelPointI, _format: bool, cb: impl FnOnce(&mut Zel)) {
        if xy.x < 0 || xy.y < 0 || xy.x as u32 >= self.size.width || xy.y as u32 >= self.size.height {
            return 
        }
        cb(self.zels.get_mut((xy.y as u32 * self.size.width + xy.x as u32) as usize).expect("if zels is correctly formed, this zel exists"))
    }

    fn bounds(&mut self) -> ZelRectI {
        Rect::new(point2(0, 0), size2(self.size.width as i32, self.size.height as i32))
    }

    fn get_font(&self) -> crate::Font {
        return Font::Normal
    }

    fn clear(&mut self) {
        self.resize(self.size)
    }
}