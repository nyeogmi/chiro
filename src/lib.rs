#![feature(type_alias_impl_trait)]

mod boxart;
pub mod modifiers;
mod shared;
mod font;
mod input;
mod screen;
mod output;
mod tileset;
pub mod minifb;

pub use boxart::{BoxArt, BoxSettings};
pub use font::Font;
pub use input::{Input, Mouse, Keyboard, MouseButton, Event};
pub use output::{FChar, FCharDraw, FString, ToFString, ToFChar, Output};
pub use shared::{Brush, Chiro, ChiroError, Color, Drawable, Eventable, ToZel, ToZelSize, Zel, At, SuperTile};

pub mod simple_io {
    pub use super::shared::SimpleIO;
}

pub mod structures {
    pub use super::input::{Event, MouseEvent, TypeEvent, PressEvent};
    pub use super::input::{TypeKey, PressKey, TypeKeyCode};
    pub use super::screen::ZelData;
}

pub mod pub_internals {
    // internals that you might still have some reason to want
    pub use super::screen::{DirtyRegion, PixelFB, Screen};
}