use euclid::*;

use crate::{shared::*, Font};

use super::{sharing::SharedMut, utils::build_rect};

#[derive(Clone)]
pub struct At<'d, D: Drawable>(
    ZelPointI, 
    SharedMut<'d, D>
);

impl<'d, D: Drawable> At<'d, D> {
    pub(super) fn new(location: ZelPointI, drawable: SharedMut<'d, D>) -> At<'d, D> {
        At(location, drawable)
    }
}

impl<'a, D: Drawable> At<'a, D> {
    pub fn at(&self, xy: impl ToZelPointI) -> At<'a, D> {
        self.1.clone().at(xy)
    }

    pub(super) fn _internally<T: Drawable>(&self, f: impl Fn(SharedMut<'a, D>) -> T) -> At<'a, T> {
        At::new(self.0, SharedMut::owned(f(self.1.clone())))
    }

    fn _putc(mut self, font: Font, c: char, clip: Option<ZelRectI>, skip: bool) -> Self {
        let w = font.char_size().width;

        font.char_to_tile(c, |local, tile| {
            let (bx, by) = self.0.to_tuple();
            let (dx, dy) = local.to_tuple();
            let point = point2(bx + dx as i32, by + dy as i32);

            if let Some(cl) = clip { 
                if !cl.contains(point) { return; } 
            }

            self.1.borrow(|d| d.raw_touch(point, true,|zel| {
                if !skip { zel.tile = tile; }
            }));
        });
        self.0.x += w as i32;
        self
    }

    pub fn putc(self, c: char) -> Self {
        let font = self.1.borrow(|d|d.get_font());
        self._putc(font, c, None, false)
    }

    pub fn touch(self) -> Self {
        let font = self.1.borrow(|d|d.get_font());
        self._putc(font, ' ', None, true)
    }

    pub fn puts(mut self, s: &str) -> Self {
        let font = self.1.borrow(|d|d.get_font());

        for c in s.chars() {
            self = self._putc(font, c, None, false)
        }
        self
    }

    pub fn fill_rect(self, other: impl ToZelPointI, c: char) -> Self {
        let font = self.1.borrow(|d| d.get_font());
        self._forall_rect(font, other, |x, clip| x._putc(font, c, Some(clip), false))
    }

    pub fn fill_rect_i(self, other: (i32, i32), c: char) -> Self {
        self.fill_rect(other, c)
    }

    pub fn touch_rect(self, other: impl ToZelPointI) -> Self {
        // use small font to fill evenly
        let font = Font::Small;
        self._forall_rect(font, other, |x, clip| x._putc(font, ' ', Some(clip), true))
    }

    pub fn touch_rect_i(self, other: (i32, i32)) -> Self {
        self.touch_rect(other)
    }

    fn _forall_rect(mut self, font: Font, other: impl ToZelPointI, mut cb: impl FnMut(Self, ZelRectI) -> Self) -> Self {
        // TODO: clip to avoid exceeding bounds
        let zeli_other = other.to_zeli();

        let big_clip = self.1.borrow(|d| d.bounds());
        let small_clip = build_rect(self.0, zeli_other);

        let internal_clip = 
            match big_clip.intersection(&small_clip) {
                None => return self,
                Some(x) => x
            };

        let old_xy = self.0;

        for x in small_clip.x_range().step_by(font.char_size().width as usize) {
            for y in small_clip.y_range().step_by(font.char_size().height as usize) {
                self.0 = point2(x, y);
                self = cb(self, internal_clip)
            }
        }
        self.0 = old_xy;
        self
    }
}