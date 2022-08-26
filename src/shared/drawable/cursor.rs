use euclid::*;

use crate::{shared::*, Font, FChar, ToFChar, ToFString};

use super::{sharing::SharedMut, utils::build_rect};

#[derive(Clone)]
pub struct At<'d, D: Drawable>{
    start: ZelPointI,
    position: ZelPointI, 
    drawable: SharedMut<'d, D>
}

impl<'d, D: Drawable> At<'d, D> {
    pub(super) fn new(start: ZelPointI, drawable: SharedMut<'d, D>) -> At<'d, D> {
        At { start, position: start, drawable }
    }
}

impl<'a, D: Drawable> At<'a, D> {
    pub fn at(&self, xy: impl ToZelPointI) -> At<'a, D> {
        self.drawable.clone().at(xy)
    }

    pub fn shift(&self, xy: impl ToZelPointI) -> At<'a, D> {
        let point = xy.to_zeli();
        At {
            start: self.start,
            position: (self.position.x + point.x, self.position.y + point.y).to_zeli(),
            drawable: self.drawable.clone(),
        }
    }

    pub fn at_i(&self, xy: (i32, i32)) -> At<'a, D> { self.at(xy) }
    pub fn shift_i(&self, xy: (i32, i32)) -> At<'a, D> { self.shift(xy) }

    pub(super) fn _internally<T: Drawable>(&self, f: impl Fn(SharedMut<'a, D>) -> T) -> At<'a, T> {
        At { start: self.start, position: self.position, drawable: SharedMut::owned(f(self.drawable.clone())) }
    }

    fn _putc(mut self, font: Font, fc: FChar, clip: Option<ZelRectI>) -> Self {
        let (w, h) = font.char_size().to_tuple();

        match fc {
            FChar::Empty => {
                font.char_to_tile(' ', |local, _| {
                    let (bx, by) = self.position.to_tuple();
                    let (dx, dy) = local.to_tuple();
                    let point = point2(bx + dx as i32, by + dy as i32);

                    if let Some(cl) = clip { 
                        if !cl.contains(point) { return; } 
                    }

                    self.drawable.borrow(|d| d.raw_touch(point, true, |_| { }));
                });
                self.position.x += w as i32;
            },

            FChar::Draw(fcd) => {
                font.char_to_tile(fcd.character, |local, tile| {
                    let (bx, by) = self.position.to_tuple();
                    let (dx, dy) = local.to_tuple();
                    let point = point2(bx + dx as i32, by + dy as i32);

                    if let Some(cl) = clip { 
                        if !cl.contains(point) { return; } 
                    }

                    self.drawable.borrow(|d| d.raw_touch(point, true, |zel| { zel.tile = tile; }));

                    if fcd.formatting.makes_changes() {
                        self.drawable.borrow(|d| d.raw_touch(point, false, |zel| {
                            fcd.formatting.apply(zel)
                        }));
                    }
                });
                self.position.x += w as i32;
            }
            FChar::Newline => {
                self.position.x = self.start.x;
                self.position.y += h as i32;
            }
        }


        self
    }

    pub fn touch(self) -> Self {
        self.put(FChar::empty())
    }

    pub fn put(mut self, s: impl ToFString) -> Self {
        let font = self.drawable.borrow(|d| d.get_font());

        for c in s.to_fchars() { self = self._putc(font, c, None) }
        self
    }

    pub fn fill_rect(self, other: impl ToZelPointI, fc: impl ToFChar) -> Self {
        let font = self.drawable.borrow(|d| d.get_font());
        self._forall_rect(font, other, |x, clip| x._putc(font, fc.to_fchar(), Some(clip)))
    }

    pub fn fill_rect_i(self, other: (i32, i32), fc: impl ToFChar) -> Self {
        self.fill_rect(other, fc)
    }

    pub fn touch_rect(self, other: impl ToZelPointI) -> Self {
        self.fill_rect(other, FChar::empty())
    }

    pub fn touch_rect_i(self, other: (i32, i32)) -> Self {
        self.touch_rect(other)
    }

    fn _forall_rect(mut self, font: Font, other: impl ToZelPointI, mut cb: impl FnMut(Self, ZelRectI) -> Self) -> Self {
        // TODO: clip to avoid exceeding bounds
        let zeli_other = other.to_zeli();

        let big_clip = self.drawable.borrow(|d| d.bounds());
        let small_clip = build_rect(self.position, zeli_other);

        let internal_clip = 
            match big_clip.intersection(&small_clip) {
                None => return self,
                Some(x) => x
            };

        let old_xy = self.position;

        for x in small_clip.x_range().step_by(font.char_size().width as usize) {
            for y in small_clip.y_range().step_by(font.char_size().height as usize) {
                self.position = point2(x, y);
                self = cb(self, internal_clip)
            }
        }
        self.position = old_xy;
        self
    }
}