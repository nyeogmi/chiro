use crate::{screen::Zel, Font};

use super::*;
use euclid::*;

pub trait Drawable {
    fn affordance(&mut self) -> Affordance;
    fn clear(&mut self);
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
    Fg(Color),
    Bg(Color),
    Click(Option<Affordance>),
    Scroll(Option<Affordance>),
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
                    i.apply_to(zel)
                }
            }
        });
        self.xy.x += w as i32;
        self
    }

    pub fn putc(self, c: char) -> Self {
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

    pub fn fg(mut self, color: impl ToColor) -> Self {
        self.modifiers.push(Modifier::Fg(color.to_color()));
        self
    }

    pub fn bg(mut self, color: impl ToColor) -> Self {
        self.modifiers.push(Modifier::Bg(color.to_color()));
        self
    }

    pub fn click(mut self, affordance: Affordance) -> Self {
        self.modifiers.push(Modifier::Click(Some(affordance)));
        self
    }

    pub fn scroll(mut self, affordance: Affordance) -> Self {
        self.modifiers.push(Modifier::Scroll(Some(affordance)));
        self
    }

    pub fn no_click(mut self) -> Self {
        self.modifiers.push(Modifier::Click(None));
        self
    }

    pub fn no_scroll(mut self) -> Self {
        self.modifiers.push(Modifier::Scroll(None));
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

impl<'a> Modifier<'a> {
    fn apply_to(&self, zel: &mut Zel) {
        match self {
            Modifier::Font(_) => {}
            Modifier::Fg(fg) => zel.fg = *fg,
            Modifier::Bg(bg) => zel.bg = *bg,
            Modifier::Click(aff) => zel.click = aff.clone(),
            Modifier::Scroll(aff) => zel.scroll = aff.clone(),
            Modifier::Arbitrary(f) => f(zel),
        }
    }
}
