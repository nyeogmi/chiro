use crate::{screen::Zel, Font};

use super::*;
use euclid::*;

pub trait Drawable {
    fn raw_view(&self, zp: ZelPointI) -> Zel;
    fn raw_at(&mut self, zp: ZelPointI) -> Option<&mut Zel>;

    fn view(&self, xy: impl ToZelPointI) -> Zel {
        self.raw_view(xy.to_zeli())
    }

    fn at(&mut self, xy: impl ToZelPointI) -> At<Self> {
        At { drawable: self, xy: xy.to_zeli(), modifiers: vec![] }
    }

    // type-coercing view/at
    fn view_i(&self, xy: (i32, i32)) -> Zel { self.view(xy) }
    fn at_i(&mut self, xy: (i32, i32)) -> At<Self> { self.at(xy) }
}

pub struct At<'a, D: ?Sized+Drawable> {
    drawable: &'a mut D,
    xy: ZelPointI,
    modifiers: Vec<Modifier<'a>>,
}

enum Modifier<'a> {
    Font(Font),
    Arbitrary(&'a dyn Fn(&mut Zel)),
}

impl<'a, D: ?Sized+Drawable> At<'a, D> {
    fn _putc(mut self, font: Font, c: char) -> Self {
        let w = font.char_size().width;

        font.char_to_tile(c, |local, tile| {
            let (bx, by) = self.xy.to_tuple();
            let (dx, dy) = local.to_tuple();

            if let Some(zel) = self.drawable.raw_at(point2(bx + dx as i32, by + dy as i32)) {
                zel.tile = tile;
                for i in self.modifiers.iter() { 
                    if let Modifier::Arbitrary(a) = i { a(zel) }
                }
            }
        });
        self.xy.x += w as i32;
        self
    }

    pub fn putc(mut self, c: char) -> Self {
        let mut font = Font::Normal;
        for i in self.modifiers.iter() {
            if let Modifier::Font(f) = i { font = *f; }
        }
        self._putc(font, c)
    }

    pub fn puts(mut self, s: &str) -> Self {
        let mut font = Font::Normal;
        for i in self.modifiers.iter() {
            if let Modifier::Font(f) = i { font = *f; }
        }
        for c in s.chars() { self = self._putc(font, c) }
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.modifiers.push(Modifier::Font(font));
        self
    }

    pub fn push_mod(mut self, modifier: &'a dyn Fn(&mut Zel)) -> Self {
        self.modifiers.push(Modifier::Arbitrary(modifier));
        self
    }

    pub fn pop_mod(mut self) -> Self {
        self.modifiers.pop();
        self
    }
}
