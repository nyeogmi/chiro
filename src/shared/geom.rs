use euclid::*;

pub struct ZelSpace;
pub struct PixelSpace;

pub(crate) const ZEL_PIXELS_X: u32 = 8;
pub(crate) const ZEL_PIXELS_Y: u32 = 8;

pub type Zel = Point2D<i32, ZelSpace>;
pub type ZelSize = Size2D<u32, ZelSpace>;

pub type ZelRect = Rect<i32, ZelSpace>;

pub type PixelSize = Size2D<u32, PixelSpace>;


pub trait ToPixelSize {
    fn to_pixels(self) -> PixelSize;
}

pub trait ToZelSize {
    fn to_zels(self) -> ZelSize;
}

impl ToPixelSize for PixelSize {
    fn to_pixels(self) -> PixelSize { self }
}

impl ToZelSize for ZelSize {
    fn to_zels(self) -> ZelSize { self }
}

impl ToPixelSize for ZelSize {
    fn to_pixels(self) -> PixelSize { 
        return PixelSize::new(self.width * ZEL_PIXELS_X, self.height * ZEL_PIXELS_Y) 
    }
}

impl ToZelSize for PixelSize {
    fn to_zels(self) -> ZelSize { 
        return ZelSize::new(self.width / ZEL_PIXELS_X, self.height / ZEL_PIXELS_Y) 
    }
}

// ZelSize alternates for constructors
impl ToZelSize for (u32, u32) {
    fn to_zels(self) -> ZelSize {
        size2(self.0, self.1)
    }
}


// NOTE: Some of these will panic, but none will panic for reasonable screen sizes, so that's OK
pub trait ToZel {
    fn to_zel(self) -> Zel;
}

// == convert the 3 signed point representations to ZelPointI ==
impl ToZel for Zel {
    fn to_zel(self) -> Zel { self }
}

impl ToZel for (i32, i32) {
    fn to_zel(self) -> Zel { point2(self.0, self.1) }
}

impl ToZel for (isize, isize) {
    fn to_zel(self) -> Zel { point2(self.0 as i32, self.1 as i32) }
}


// == convert the 2 unsigned point representations to ZelPointI ==
impl ToZel for (u32, u32) {
    fn to_zel(self) -> Zel { point2(self.0 as i32, self.1 as i32) }
}

impl ToZel for (usize, usize) {
    fn to_zel(self) -> Zel { point2(self.0 as i32, self.1 as i32) }
}