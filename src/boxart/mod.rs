mod charset;
mod complex;
mod simple;

pub use complex::BoxArt;
pub(crate) use simple::draw_box;
pub use simple::{Border, BoxSettings};


use enum_map::Enum;

#[derive(Debug, Enum, Eq, PartialEq, Hash, Clone, Copy)]
pub enum BoxSide {
    Top = 0, Right = 1, Bottom = 2, Left = 3,
}