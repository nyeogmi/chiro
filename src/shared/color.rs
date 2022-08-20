#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u32);

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color { Color(u32::from_le_bytes([b, g, r, 255])) }
    pub const fn transparent() -> Color { Color(0) }

    pub fn is_transparent(&self) -> bool { return self.0.to_le_bytes()[3] != 255 }
    pub fn is_opaque(&self) -> bool { return !self.is_transparent() }

    pub(crate) fn to_rgb(&self) -> (u8, u8, u8) {
        let bytes = self.0.to_le_bytes();
        (bytes[0], bytes[1], bytes[2])
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::transparent()
    }
}

pub trait ToColor {
    fn to_color(self) -> Color;
}

impl ToColor for Color {
    fn to_color(self) -> Color { self }
}

impl ToColor for u32 {
    fn to_color(self) -> Color {
        // ex: 0xff0080
        let bytes = self.to_le_bytes();
        return Color::rgb(bytes[2], bytes[1], bytes[0])
    }
}

impl ToColor for (u8, u8, u8) {
    fn to_color(self) -> Color {
        return Color::rgb(self.0, self.1, self.2)
    }
}