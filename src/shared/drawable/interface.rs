use crate::{shared::*, screen::Zel, Font};
use super::At;

use super::sharing::SharedMut;

pub trait Drawable: Sized {
    fn affordance(&mut self) -> Affordance;
    fn get_font(&self) -> Font;

    fn bounds(&mut self) -> ZelRectI;
    fn raw_view(&self, zp: ZelPointI) -> Zel;
    fn raw_touch(&mut self, zp: ZelPointI, format: bool, modify: impl FnOnce(&mut Zel));

    fn view(&self, xy: impl ToZelPointI) -> Zel {
        self.raw_view(xy.to_zeli())
    }

    fn view_i(&self, xy: (i32, i32)) -> Zel { self.view(xy) }

    // get a cursor
    fn at(&mut self, xy: impl ToZelPointI) -> At<Self> {
        At::new(xy.to_zeli(), SharedMut::wrap(self))
    }

    // build modifiers (reexport from SharedMut)
    // unfortunately this has to be done here as these are trait-level functions
    fn offset(&mut self, xy: impl ToZelPointI) -> Offset<Self> {
        SharedMut::wrap(self).offset(xy)
    }

    fn clip(&mut self, xy0: impl ToZelPointI, xy1: impl ToZelPointI) -> Clip<Self> {
        SharedMut::wrap(self).clip(xy0, xy1)
    }

    fn font(&mut self, font: Font) -> SetFont<Self> { SharedMut::wrap(self).font(font) }
    fn fg(&mut self, color: impl ToColor) -> SetFg<Self> { SharedMut::wrap(self).fg(color) }
    fn bg(&mut self, color: impl ToColor) -> SetBg<Self> { SharedMut::wrap(self).bg(color) }
    fn click(&mut self, affordance: Affordance) -> SetClick<Self> { SharedMut::wrap(self).click(affordance) }
    fn scroll(&mut self, affordance: Affordance) -> SetScroll<Self> { SharedMut::wrap(self).scroll(affordance) }
    fn no_click(&mut self) -> SetClick<Self> { SharedMut::wrap(self).no_click() }
    fn no_scroll(&mut self) -> SetScroll<Self> { SharedMut::wrap(self).no_scroll() }

    // type-coercing 
    fn at_i(&mut self, xy: (i32, i32)) -> At<Self> { SharedMut::wrap(self).at_i(xy) }
    fn offset_i(&mut self, xy: (i32, i32)) -> Offset<Self> { SharedMut::wrap(self).offset_i(xy) }
    fn clip_i(&mut self, xy0: (i32, i32), xy1: (i32, i32)) -> Clip<Self> {
        SharedMut::wrap(self).clip_i(xy0, xy1)
    }

    // drawing
    fn fill(&mut self, c: char) {
        let bounds = self.bounds();
        self.at(bounds.min()).fill_rect(bounds.max(), c);
    }

    fn clear(&mut self) {
        self.fill(' ')
    }
}
