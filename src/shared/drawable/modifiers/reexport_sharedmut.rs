use crate::{shared::drawable::sharing::SharedMut, Drawable};
use super::*;

impl<'d, D: Drawable> SharedMut<'d, D> {
    // == modifiers etc ==
    // get a cursor
    pub fn at(self, xy: impl ToZelPointI) -> At<'d, D> {
        At::new(xy.to_zeli(), self)
    }

    // build modifiers
    pub fn offset(self, xy: impl ToZelPointI) -> Offset<'d, D> {
        Offset(xy.to_zeli(), self)
    }

    pub fn clip(self, xy0: impl ToZelPointI, xy1: impl ToZelPointI) -> Clip<'d, D> {
        Clip(build_rect(xy0.to_zeli(), xy1.to_zeli()), self)
    }

    pub fn font(self, font: Font) -> SetFont<'d, D> { SetFont(font, self) }
    pub fn fg(self, color: impl ToColor) -> SetFg<'d, D> { SetFg(color.to_color(), self) }
    pub fn bg(self, color: impl ToColor) -> SetBg<'d, D> { SetBg(color.to_color(), self) }
    pub fn click(self, affordance: Affordance) -> SetClick<'d, D> { SetClick(Some(affordance), self) }
    pub fn scroll(self, affordance: Affordance) -> SetScroll<'d, D> { SetScroll(Some(affordance), self) }
    pub fn no_click(self) -> SetClick<'d, D> { SetClick(None, self) }
    pub fn no_scroll(self) -> SetScroll<'d, D> { SetScroll(None, self) }

    // type-coercing 
    pub fn at_i(self, xy: (i32, i32)) -> At<'d, D> { self.at(xy) }
    pub fn offset_i(self, xy: (i32, i32)) -> Offset<'d, D> { self.offset(xy) }
    pub fn clip_i(self, xy0: (i32, i32), xy1: (i32, i32)) -> Clip<'d, D> {
        self.clip(xy0, xy1)
    }
}