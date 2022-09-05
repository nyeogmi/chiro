use euclid::*;

use crate::{shared::*, Font, FChar, ToFChar, ToFString, boxart::{draw_box, BoxSettings}};

use super::brush::Brush;

pub struct At<'d, D: Drawable>{
    start: Zel,
    position: Zel, 
    drawable: Shared<'d, D>,
}

impl<'d, D: Drawable> Clone for At<'d, D> {
    fn clone(&self) -> Self {
        At { start: self.start, position: self.position, drawable: self.drawable.clone() }
    }
}

impl<'d, D: Drawable> At<'d, D> {
    pub(crate) fn new(start: Zel, drawable: Shared<'d, D>) -> At<'d, D> {
        At { start, position: start, drawable }
    }
}

impl<'a, D: Drawable> At<'a, D> {
    // == reexports from drawable == 
    pub fn affordance(&mut self) -> Affordance { self.drawable.borrow(|d| d.affordance()) }
    pub fn get_font(&self) -> Font { self.drawable.borrow(|d| d.get_font()) }

    // == cursor stuff ==
    pub fn at(&self, xy: impl ToZel) -> At<'a, D> {
        At::new(xy.to_zel(), self.drawable.clone())
    }


    pub fn shifted(&self, xy: impl ToZel) -> At<'a, D> {
        let point = xy.to_zel();
        At {
            start: self.start,
            position: (self.position.x + point.x, self.position.y + point.y).to_zel(),
            drawable: self.drawable.clone(),
        }
    }

    pub fn at_i(&self, xy: (i32, i32)) -> At<'a, D> { self.at(xy) }
    pub fn shifted_i(&self, xy: (i32, i32)) -> At<'a, D> { self.shifted(xy) }

    pub(crate) fn map<T: Drawable>(&self, f: impl Fn(Brush<'a, D>) -> Brush<'a, T>) -> At<'a, T> {
        let brush = Brush { drawable: self.drawable.clone() };
        At { start: self.start, position: self.position, drawable: f(brush).drawable }
    }

    fn _putc(mut self, font: Font, fc: FChar, clip: Option<ZelRect>) -> Self {
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

                    self.drawable.borrow(|d| d.touch(point, true, |_| { }));
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

                    self.drawable.borrow(|d| d.touch(point, true, |zel| { 
                        zel.tile = tile; 
                    }));

                    if fcd.formatting.makes_changes() {
                        self.drawable.borrow(|d| d.touch(point, false, |zel| { fcd.formatting.apply(zel) }));
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

    pub fn put_st(self, st: SuperTile) -> Self {
        self.drawable.borrow(|d| d.deposit_supertile(self.position, st));
        self
    }

    pub fn draw_rect(self, other: impl ToZel) -> Self {
        self.draw_rect_ext(other, BoxSettings::default())
    }

    pub fn draw_rect_ext(self, other: impl ToZel, settings: BoxSettings) -> Self {
        let (mut x1, mut y1) = other.to_zel().to_tuple();
        x1 -= self.position.x;
        y1 -= self.position.y;
        draw_box(&self, point2(0, 0), point2(x1, y1), settings);
        self
    }

    pub fn fill_rect(self, other: impl ToZel, fc: impl ToFChar) -> Self {
        let font = self.drawable.borrow(|d| d.get_font());
        self._forall_rect(font, other, |x, clip| x._putc(font, fc.to_fchar(), Some(clip)))
    }

    pub fn fill_rect_i(self, other: (i32, i32), fc: impl ToFChar) -> Self {
        self.fill_rect(other, fc)
    }

    pub fn touch_rect(self, other: impl ToZel) -> Self {
        self.fill_rect(other, FChar::empty())
    }

    pub fn touch_rect_i(self, other: (i32, i32)) -> Self {
        self.touch_rect(other)
    }

    fn _forall_rect(mut self, font: Font, other: impl ToZel, mut cb: impl FnMut(Self, ZelRect) -> Self) -> Self {
        // TODO: clip to avoid exceeding bounds
        let zeli_other = other.to_zel();

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