use std::collections::HashMap;

use crate::{Drawable, shared::{At, ZelPointI, ToZelPointI, build_rect}, font::decode_cp437, Font};

pub struct BoxArt {
    font: Font,
    content: HashMap<ZelPointI, u8>,
}

enum BoxSide {
    Up = 0, Right = 1, Down = 2, Left = 3,
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
            if let Some(bc) = Self::box_char(*value) {
                cursor.font(self.font).shifted((xy.x * sz_x, xy.y * sz_y)).put(bc);
            }
        }
    }

    pub fn draw_box(&mut self, top: impl ToZelPointI, bot: impl ToZelPointI, double_border: bool) {
        let mut top = top.to_zeli();
        let mut bot = bot.to_zeli();
        let zel_dims = self.font.char_size();
        top.x /= zel_dims.width as i32;
        top.y /= zel_dims.height as i32;
        bot.x /= zel_dims.width as i32;
        bot.y /= zel_dims.height as i32;

        let rect = build_rect(top, bot);

        if rect.size.width <= 1 {
            for y in rect.min_y()..rect.max_y() - 1 {
                self.add((rect.min_x(), y), BoxSide::Down, double_border)
            }

            for y in rect.min_y() + 1..rect.max_y() {
                self.add((rect.min_x(), y), BoxSide::Up, double_border)
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
            self.add((rect.min_x(), y), BoxSide::Down, double_border);
            self.add((rect.max_x() - 1, y), BoxSide::Down, double_border);
        }

        for y in rect.min_y() + 1..rect.max_y() {
            self.add((rect.min_x(), y), BoxSide::Up, double_border);
            self.add((rect.max_x() - 1, y), BoxSide::Up, double_border);
        }
    }

    fn add(&mut self, at: (i32, i32), side: BoxSide, double_border: bool) {
        let norm_side = 3 - side as u8;
        let existing = self.content.get(&at.to_zeli()).cloned().unwrap_or(0);
        let new = existing | (1 << (2 * norm_side + if double_border { 1 } else { 0 }));
        self.content.insert(at.to_zeli(), new);
    }

    fn box_char(mask: u8) -> Option<char> {
        // mask is one byte per edge
        //    N E S W
        // 0b00000000
        //
        // 00: no line
        // 01: single line
        // 10: double line
        // 11: double line
        // this bit fuckery makes sure that 2 becomes 3
        Some(match mask | ((mask & 0b10101010) >> 1) {
            0b00_00_00_00 => return None,
            0b01_00_00_00 => 0xc1,  // optional case
            0b00_00_01_00 => 0xc2,  // optional case
            0b01_00_01_00 |
            0b11_00_01_00 |
            0b01_00_11_00 =>
                0xb3,
            0b01_00_01_01 =>
                0xb4,
            0b01_00_01_11 =>
                0xb5,
            0b11_00_11_01 |
            0b01_00_11_01 |
            0b11_00_01_01 =>
                0xb6,
            0b00_00_11_01 =>
                0xb7,
            0b00_00_01_11 =>
                0xb8,
            0b11_00_11_11 |
            0b01_00_11_11 |
            0b11_00_01_11 =>
                0xb9,
            0b11_00_00_00 => 0xd0,  // optional case
            0b00_00_11_00 => 0xd2,  // optional case
            0b11_00_11_00 =>
                0xba,
            0b00_00_11_11 =>
                0xbb,
            0b11_00_00_11 =>
                0xbc,
            0b11_00_00_01 =>
                0xbd,
            0b01_00_00_11 =>
                0xbe,
            0b00_00_01_01 =>
                0xbf,
            0b01_01_00_00 =>
                0xc0,
            0b01_01_00_01 |
            0b01_11_00_01 |
            0b01_01_00_11 =>
                0xc1,
            0b00_01_01_01 |
            0b00_11_01_01 |
            0b00_01_01_11 =>
                0xc2,
            0b01_01_01_00 =>
                0xc3,
            0b00_01_00_00 => 0xc3,  // optional case
            0b00_00_00_01 => 0xb4,  // optional case
            0b00_01_00_01 |
            0b00_11_00_01 |
            0b00_01_00_11 =>
                0xc4,
            0b01_01_01_01 |
            0b11_01_01_01 |
            0b01_11_01_01 |
            0b01_01_11_01 |
            0b01_01_01_11 |
            0b11_11_01_01 |
            0b01_11_11_01 |
            0b01_01_11_11 |
            0b11_01_01_11 =>
                0xc5,
            0b01_11_01_00 =>
                0xc6,
            0b11_01_11_00 |
            0b01_01_11_00 |
            0b11_01_01_00 =>
                0xc7,
            0b11_11_00_00 =>
                0xc8,
            0b00_11_11_00 =>
                0xc9,
            0b11_11_00_11 |
            0b11_01_00_11 |
            0b11_11_00_01 =>
                0xca,
            0b00_11_11_11 |
            0b00_01_11_11 |
            0b00_11_11_01 =>
                0xcb,
            0b11_11_11_00 |
            0b01_11_11_00 |
            0b11_11_01_00 =>
                0xcc,
            0b00_11_00_00 => 0xc6,  // optional case
            0b00_00_00_11 => 0xb5,  // optional case
            0b00_11_00_11 =>
                0xcd,
            0b11_11_11_11 |
            0b01_11_11_11 |
            0b11_01_11_11 |
            0b11_11_01_11 |
            0b11_11_11_01 =>
                0xce,
            0b01_11_00_11 =>
                0xcf,
            0b11_01_00_01 =>
                0xd0,
            0b00_11_01_11 =>
                0xd1,
            0b00_01_11_01 =>
                0xd2,
            0b11_01_00_00 =>
                0xd3,
            0b01_11_00_00 =>
                0xd4,
            0b00_11_01_00 =>
                0xd5,
            0b00_01_11_00 =>
                0xd6,
            0b11_01_11_01 =>
                0xd7,
            0b01_11_01_11 =>
                0xd8,
            0b01_00_00_01 =>
                0xd9,
            0b00_01_01_00 =>
                0xda,
            _ => unreachable!("confused about mask: {}", mask)
        }).map(decode_cp437)
    }
}