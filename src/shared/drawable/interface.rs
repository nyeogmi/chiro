use crate::ToFChar;
use crate::{shared::*, screen::ZelData, Font};
use crate::modifiers::*;
use super::At;

pub trait Drawable: Sized {
    fn affordance(&mut self) -> Affordance;
    fn get_font(&self) -> Font;

    fn bounds(&mut self) -> ZelRect;
    fn raw_view(&self, zp: Zel) -> ZelData;
    fn raw_touch(&mut self, zp: Zel, format: bool, modify: impl FnOnce(&mut ZelData));

    fn view(&self, xy: impl ToZel) -> ZelData {
        self.raw_view(xy.to_zel())
    }

    fn view_i(&self, xy: (i32, i32)) -> ZelData { self.view(xy) }

    // get a cursor
    fn at(&mut self, xy: impl ToZel) -> At<Self> {
        At::new(xy.to_zel(), Shared::wrap(self))
    }

    // build modifiers (reexport from SharedMut)
    // unfortunately this has to be done here as these are trait-level functions
    fn offset(&mut self, xy: impl ToZel) -> Offset<Self> {
        Shared::wrap(self).offset(xy)
    }

    fn clip(&mut self, xy0: impl ToZel, xy1: impl ToZel) -> Clip<Self> {
        Shared::wrap(self).clip(xy0, xy1)
    }

    fn font(&mut self, font: Font) -> SetFont<Self> { Shared::wrap(self).font(font) }
    fn fg(&mut self, color: impl ToColor) -> SetFg<Self> { Shared::wrap(self).fg(color) }
    fn bg(&mut self, color: impl ToColor) -> SetBg<Self> { Shared::wrap(self).bg(color) }
    fn click(&mut self, affordance: Affordance) -> SetClick<Self> { Shared::wrap(self).click(affordance) }
    fn scroll(&mut self, affordance: Affordance) -> SetScroll<Self> { Shared::wrap(self).scroll(affordance) }
    fn no_click(&mut self) -> SetClick<Self> { Shared::wrap(self).no_click() }
    fn no_scroll(&mut self) -> SetScroll<Self> { Shared::wrap(self).no_scroll() }

    // type-coercing 
    fn at_i(&mut self, xy: (i32, i32)) -> At<Self> { Shared::wrap(self).at_i(xy) }
    fn offset_i(&mut self, xy: (i32, i32)) -> Offset<Self> { Shared::wrap(self).offset_i(xy) }
    fn clip_i(&mut self, xy0: (i32, i32), xy1: (i32, i32)) -> Clip<Self> {
        Shared::wrap(self).clip_i(xy0, xy1)
    }

    // drawing
    fn fill(&mut self, fc: impl ToFChar) {
        let bounds = self.bounds();
        self.at(bounds.min()).fill_rect(bounds.max(), fc);
    }

    fn clear(&mut self) {
        self.fill(' ')
    }
}
