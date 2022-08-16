use euclid::*;

pub struct ZelSpace;
pub struct PixelSpace;

pub type ZelPoint = Point2D<u32, ZelSpace>;
pub type ZelSize = Size2D<u32, ZelSpace>;

pub type PixelSize = Size2D<u32, PixelSpace>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Affordance(u32);


pub trait ToPixelSize {
    fn to_pixels(self) -> PixelSize;
}

pub trait ToZelSize {
    fn to_zels(self) -> ZelSize;
}

pub(crate) const ZEL_PIXELS_X: u32 = 8;
pub(crate) const ZEL_PIXELS_Y: u32 = 8;

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