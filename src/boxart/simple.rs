use enum_map::{EnumMap};

use crate::{shared::{At, ToZel, build_rect}, Drawable};

use super::{BoxSide, charset::box_char};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Border { Single, Double, }

#[derive(Clone, Copy, Default)]
pub struct BoxSettings {
    borders: EnumMap<BoxSide, Border>
} 

impl Default for Border {
    fn default() -> Self { Border::Single }
}

impl BoxSettings {
    pub fn single() -> BoxSettings {
        BoxSettings::default()
    }

    pub fn double() -> BoxSettings {
        let mut settings = BoxSettings::default();

        settings.borders[BoxSide::Top] = Border::Double;
        settings.borders[BoxSide::Bottom] = Border::Double;
        settings.borders[BoxSide::Left] = Border::Double;
        settings.borders[BoxSide::Right] = Border::Double;

        settings
    }

    pub fn double_top(mut self) -> BoxSettings { self.borders[BoxSide::Top] = Border::Double; self }
    pub fn double_bottom(mut self) -> BoxSettings { self.borders[BoxSide::Bottom] = Border::Double; self }
    pub fn double_left(mut self) -> BoxSettings { self.borders[BoxSide::Left] = Border::Double; self }
    pub fn double_right(mut self) -> BoxSettings { self.borders[BoxSide::Right] = Border::Double; self }

    pub fn single_top(mut self) -> BoxSettings { self.borders[BoxSide::Top] = Border::Single; self }
    pub fn single_bottom(mut self) -> BoxSettings { self.borders[BoxSide::Bottom] = Border::Single; self }
    pub fn single_left(mut self) -> BoxSettings { self.borders[BoxSide::Left] = Border::Single; self }
    pub fn single_right(mut self) -> BoxSettings { self.borders[BoxSide::Right] = Border::Single; self }
}


pub(crate) fn draw_box(cursor: &At<impl Drawable>, top: impl ToZel, bot: impl ToZel, settings: BoxSettings) {
    let font = cursor.get_font();
    let sz = font.char_size();
    let sz_x = sz.width as i32;
    let sz_y = sz.height as i32;

    let mut top = top.to_zel();
    let mut bot = bot.to_zel();
    top.x /= sz_x;
    top.y /= sz_y;
    bot.x /= sz_x;
    bot.y /= sz_y;

    let r = build_rect(top, bot);

    // inclusive, because it simplifies some math
    let (x0, y0, x1, y1) = (r.min_x(), r.min_y(), r.max_x() - 1, r.max_y() - 1);

    if r.size.width <= 1 {
        let use_double = 
            settings.borders[BoxSide::Left] == Border::Double ||
            settings.borders[BoxSide::Right] == Border::Double;

        let mut up_crumb: u8 = 0b01_00_00_00;
        let mut down_crumb: u8 = 0b00_00_01_00;
        if use_double {
            up_crumb |= up_crumb << 1;
            down_crumb |= down_crumb << 1;
        }

        draw_vertical_line(
            &cursor, 
            x0, y0, y1,
            down_crumb, down_crumb | up_crumb, up_crumb
        );
        return;
    }

    if r.size.height <= 1 {
        let use_double = 
            settings.borders[BoxSide::Top] == Border::Double ||
            settings.borders[BoxSide::Bottom] == Border::Double;

        let mut left_crumb: u8 = 0b00_00_00_01;
        let mut right_crumb: u8 = 0b00_01_00_00;
        if use_double {
            left_crumb |= left_crumb << 1;
            right_crumb |= right_crumb << 1;
        }
        draw_horizontal_line(
            &cursor, 
            y0, x0, x1, 
            right_crumb, right_crumb | left_crumb, left_crumb
        );
        return;
    }

    let use_double_left = settings.borders[BoxSide::Left] == Border::Double;
    let use_double_right = settings.borders[BoxSide::Right] == Border::Double;
    let use_double_up = settings.borders[BoxSide::Top] == Border::Double;
    let use_double_down = settings.borders[BoxSide::Bottom] == Border::Double;

    // draw left border
    {
        let mut middle: u8 = 0b01_00_01_00;
        if use_double_left { middle |= middle << 1 }

        draw_vertical_line(
            &cursor, 
            x0, y0, y1,
            0, middle, 0,
        );
    }

    // draw right border
    {
        let mut middle: u8 = 0b01_00_01_00;
        if use_double_right { middle |= middle << 1 }

        draw_vertical_line(
            &cursor, 
            x1, y0, y1,
            0, middle, 0,
        );
    }

    // draw top border
    {
        let mut middle: u8 = 0b00_01_00_01;
        if use_double_up { middle |= middle << 1 }

        draw_horizontal_line(
            &cursor, 
            y0, x0, x1, 
            0, middle, 0,
        );
    }

    // draw bottom border
    {
        let mut middle: u8 = 0b00_01_00_01;
        if use_double_down { middle |= middle << 1 }

        draw_horizontal_line(
            &cursor, 
            y1, x0, x1,
            0, middle, 0,
        );
    }

    // draw corners
    let mut ul = 0b00_00_00_00;
    ul |= if use_double_left { 0b00_00_11_00 } else { 0b00_00_01_00 };
    ul |= if use_double_up { 0b00_11_00_00 } else { 0b00_01_00_00 };
    draw_box_char(&cursor, (x0, y0), ul);

    let mut ur = 0b00_00_00_00;
    ur |= if use_double_right { 0b00_00_11_00 } else { 0b00_00_01_00 };
    ur |= if use_double_up { 0b00_00_00_11 } else { 0b00_00_00_01 };
    draw_box_char(&cursor, (x1, y0), ur);

    let mut dl = 0b00_00_00_00;
    dl |= if use_double_left { 0b11_00_00_00 } else { 0b01_00_00_00 };
    dl |= if use_double_down { 0b00_11_00_00 } else { 0b00_01_00_00 };
    draw_box_char(&cursor, (x0, y1), dl);

    let mut dr = 0b00_00_00_00;
    dr |= if use_double_right { 0b11_00_00_00 } else { 0b01_00_00_00 };
    dr |= if use_double_down { 0b00_00_00_11 } else { 0b00_00_00_01 };
    draw_box_char(&cursor, (x1, y1), dr);
}


fn draw_vertical_line(cursor: &At<impl Drawable>, x: i32, min_y: i32, max_y: i32, cap_top: u8, middle: u8, cap_bot: u8) {
    draw_box_char(cursor, (x, min_y), cap_top);
    if max_y <= min_y { return; }
    draw_box_char(cursor, (x, max_y), cap_bot);

    for y in min_y + 1..= max_y - 1 {
        draw_box_char(cursor, (x, y), middle);
    }
}


fn draw_horizontal_line(cursor: &At<impl Drawable>, y: i32, min_x: i32, max_x: i32, cap_left: u8, middle: u8, cap_right: u8) {
    draw_box_char(cursor, (min_x, y), cap_left);
    if max_x <= min_x { return; }
    draw_box_char(cursor, (max_x, y), cap_right);

    for x in min_x + 1..=max_x - 1 {
        draw_box_char(cursor, (x, y), middle);
    }
}


fn draw_box_char(cursor: &At<impl Drawable>, (x, y): (i32, i32), c: u8) {
    let font = cursor.get_font();
    let (sz_x, sz_y) = font.char_size().to_tuple();
    if let Some(bc) = box_char(c) {
        cursor.shifted((x * sz_x as i32, y * sz_y as i32)).put(bc);
    }
}