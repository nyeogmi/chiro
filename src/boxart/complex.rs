use std::collections::{HashMap, hash_map::Entry};

use crate::{Drawable, shared::{At, Zel, ToZel, build_rect}, Font};

use super::{charset::box_char, BoxSide};

pub struct BoxArt {
    font: Font,
    content: HashMap<Zel, u8>,
}

impl BoxArt {
    pub fn new(font: Font) -> BoxArt {
        BoxArt {
            font,
            content: HashMap::new(),
        }
    }

    pub fn draw<'d>(&self, cursor: At<'d, impl Drawable+'d>) {
        let sz = self.font.char_size();
        let sz_x = sz.width as i32;
        let sz_y = sz.height as i32;

        for (xy, value) in self.content.iter() {
            if let Some(bc) = box_char(*value) {
                cursor.font(self.font).shifted((xy.x * sz_x, xy.y * sz_y)).put(bc);
            }
        }
    }

    pub fn add_box(&mut self, top: impl ToZel, bot: impl ToZel, double_border: bool) {
        let mut top = top.to_zel();
        let mut bot = bot.to_zel();
        let zel_dims = self.font.char_size();
        top.x /= zel_dims.width as i32;
        top.y /= zel_dims.height as i32;
        bot.x /= zel_dims.width as i32;
        bot.y /= zel_dims.height as i32;

        let rect = build_rect(top, bot);

        if rect.size.width <= 1 {
            for y in rect.min_y()..rect.max_y() - 1 {
                self.add((rect.min_x(), y), BoxSide::Bottom, double_border)
            }

            for y in rect.min_y() + 1..rect.max_y() {
                self.add((rect.min_x(), y), BoxSide::Top, double_border)
            }
            return;
        }

        if rect.size.height <= 1 {
            for x in rect.min_x()..rect.max_x() - 1 {
                self.add((x, rect.min_y()), BoxSide::Right, double_border)
            }
            for x in rect.min_x() + 1..rect.max_x() {
                self.add((x, rect.min_y()), BoxSide::Left, double_border)
            }

            return;
        }

        for x in rect.min_x()..rect.max_x() - 1 {
            self.add((x, rect.min_y()), BoxSide::Right, double_border);
            self.add((x, rect.max_y() - 1), BoxSide::Right, double_border);
        }

        for x in rect.min_x() + 1..rect.max_x() {
            self.add((x, rect.min_y()), BoxSide::Left, double_border);
            self.add((x, rect.max_y() - 1), BoxSide::Left, double_border);
        }

        for y in rect.min_y()..rect.max_y() - 1 {
            self.add((rect.min_x(), y), BoxSide::Bottom, double_border);
            self.add((rect.max_x() - 1, y), BoxSide::Bottom, double_border);
        }

        for y in rect.min_y() + 1..rect.max_y() {
            self.add((rect.min_x(), y), BoxSide::Top, double_border);
            self.add((rect.max_x() - 1, y), BoxSide::Top, double_border);
        }
    }

    fn add(&mut self, at: (i32, i32), side: BoxSide, double_border: bool) {
        let norm_side = 3 - side as u8;

        let new = 1 << (2 * norm_side + if double_border { 1 } else { 0 });
        match self.content.entry(at.to_zel()) {
            Entry::Occupied(mut o) => { *o.get_mut() |= new; }
            Entry::Vacant(v) => { v.insert(new); }
        };
    }
}