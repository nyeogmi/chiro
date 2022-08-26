#![feature(type_alias_impl_trait)]

mod boxart;
mod shared;
mod font;
mod input;
mod screen;
mod output;
mod tileset;
mod window;

pub use boxart::BoxArt;
pub use font::Font;
pub use input::{Event, Mouse, Keyboard, MouseButton};
pub use output::{FChar, FString, ToFString, ToFChar, Output};
pub use shared::{Color, Drawable, Eventable};
pub use window::Window;