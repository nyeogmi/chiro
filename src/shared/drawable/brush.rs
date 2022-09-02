use crate::{shared::*, Font, ToFChar, screen::ZelData, modifiers::*};

pub struct Brush<'d, D: Drawable> {
    pub(crate) drawable: Shared<'d, D>
}

impl<'d, D: Drawable> Clone for Brush<'d, D> {
    fn clone(&self) -> Self {
        Self { drawable: self.drawable.clone() }
    }
}

impl <'a, D: Drawable> Brush<'a, D> {
    // == reexports from drawable == 
    pub fn affordance(&self) -> Affordance { self.drawable.borrow(|d| d.affordance()) }
    pub fn get_font(&self) -> Font { self.drawable.borrow(|d| d.get_font()) }

    // == brush stuff ==
    pub fn bounds(&self) -> ZelRect { self.drawable.borrow(|x| x.bounds()) }

    pub(crate) fn map<T: Drawable>(&self, builder: impl Fn(Shared<'a, D>) -> T ) -> Brush<'a, T> {
        Brush {
            drawable: Shared::owned(builder(self.drawable.clone()))
        }
    }

    pub fn view(&self, xy: impl ToZel) -> ZelData {
        let xy = xy.to_zel();
        self.drawable.borrow(|d| d.raw_view(xy))
    }

    pub fn view_i(&self, xy: (i32, i32)) -> ZelData { self.view(xy) }

    pub fn touch(&self, xy: impl ToZel, format: bool, modify: impl FnOnce(&mut ZelData)) {
        let xy = xy.to_zel();
        self.drawable.borrow(|d| d.raw_touch(xy, format, modify))
    }

    pub fn touch_i(&self, xy: (i32, i32), format: bool, modify: impl FnOnce(&mut ZelData)) {
        self.touch(xy, format, modify)
    }

    // get a cursor
    pub fn at(&self, xy: impl ToZel) -> At<'a, D> {
        At::new(xy.to_zel(), self.drawable.clone())
    }

    // build modifiers (reexport from SharedMut)
    // unfortunately this has to be done here as these are trait-level functions
    pub fn offset(&self, xy: impl ToZel) -> Brush<'a, Offset<'a, D>> {
        let xy = xy.to_zel();
        self.map(|x| Offset(xy, x))
    }

    pub fn clip(&self, xy0: impl ToZel, xy1: impl ToZel) -> Brush<'a, Clip<'a, D>> {
        let xy0 = xy0.to_zel();
        let xy1 = xy1.to_zel();
        let rect = build_rect(xy0, xy1);
        self.map(|x| Clip(rect, x))
    }

    pub fn font(&self, font: Font) -> Brush<'a, SetFont<'a, D>> { 
        self.map(|x| SetFont(font, x))
    }
    pub fn fg(&self, color: impl ToColor) -> Brush<'a, SetFg<'a, D>> { 
        let color = color.to_color();
        self.map(|x| SetFg(color, x))
    }
    pub fn bg(&self, color: impl ToColor) -> Brush<'a, SetBg<'a, D>> { 
        let color = color.to_color();
        self.map(|x| SetBg(color, x))
    }
    pub fn click(&self, affordance: Affordance) -> Brush<'a, SetClick<'a, D>> { 
        self.map(|x| SetClick(Some(affordance), x))
    }
    pub fn scroll(&self, affordance: Affordance) -> Brush<'a, SetScroll<'a, D>> { 
        self.map(|x| SetScroll(Some(affordance), x))
    }
    pub fn no_click(&self) -> Brush<'a, SetClick<'a, D>> { 
        self.map(|x| SetClick(None, x))
    }
    pub fn no_scroll(&self) -> Brush<'a, SetScroll<'a, D>> { 
        self.map(|x| SetScroll(None, x))
    }

    // type-coercing 
    pub fn at_i(&self, xy: (i32, i32)) -> At<'a, D> { 
        self.at(xy)
    }
    pub fn offset_i(&self, xy: (i32, i32)) -> Brush<'a, Offset<'a, D>> { 
        self.offset(xy)
    }
    pub fn clip_i(&self, xy0: (i32, i32), xy1: (i32, i32)) -> Brush<'a, Clip<'a, D>> {
        self.clip(xy0, xy1)
    }

    // drawing
    pub fn fill(&self, fc: impl ToFChar) {
        let bounds = self.bounds();
        self.at(bounds.min()).fill_rect(bounds.max(), fc);
    }

    pub fn clear(&self) {
        self.fill(' ')
    }
}