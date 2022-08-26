mod charset;
mod complex;
mod simple;

#[derive(Debug, Enum, Eq, PartialEq, Hash, Clone, Copy)]
pub enum BoxSide {
    Up = 0, Right = 1, Down = 2, Left = 3,
}

pub use complex::BoxArt;
use enum_map::Enum;
pub(crate) use simple::draw_box;
pub use simple::{Border, Settings};
