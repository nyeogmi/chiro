use crate::{shared::*, screen::ZelData, Font};
use super::At;
use super::brush::Brush;

pub trait Drawable: Sized {
    fn affordance(&mut self) -> Affordance;
    fn get_font(&self) -> Font;

    fn bounds(&self) -> ZelRect;
    fn raw_view(&self, zp: Zel) -> ZelData;
    fn raw_touch(&mut self, zp: Zel, format: bool, modify: impl FnOnce(&mut ZelData));

    // reexport certain Brush functions for Drawable
    fn draw(&mut self) -> Brush<Self> {
        Brush { drawable: Shared::wrap(self) }
    }

    fn view(&self, xy: impl ToZel) -> ZelData {
        self.raw_view(xy.to_zel())
    }
    fn view_i(&self, xy: (i32, i32)) -> ZelData { self.view(xy) }

    fn touch(&mut self, xy: impl ToZel, format: bool, modify: impl FnOnce(&mut ZelData)) {
        self.draw().touch(xy, format, modify)
    }

    fn touch_i(&mut self, xy: (i32, i32), format: bool, modify: impl FnOnce(&mut ZelData)) { 
        self.touch(xy, format, modify) 
    }

    fn at(&mut self, xy: impl ToZel) -> At<Self> {
        self.draw().at(xy)
    }

    fn at_i(&mut self, xy: (i32, i32)) -> At<Self> {
        self.at(xy)
    }
}
