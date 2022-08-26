// this probably belongs in Drawable, but the errors are aggressively terrible if the path is long
// so I've moved it up here

use crate::{shared::*, Font};
use crate::shared::Shared;

mod reexport_cursor;
mod reexport_sharedmut;
mod trait_impl;

#[derive(Clone)]
pub struct Offset<'d, D: Drawable>(Zel, Shared<'d, D>);

#[derive(Clone)]
pub struct Clip<'d, D: Drawable>(ZelRect, Shared<'d, D>);

#[derive(Clone)]
pub struct SetFont<'d, D: Drawable>(Font, Shared<'d, D>);

#[derive(Clone)]
pub struct SetFg<'d, D: Drawable>(Color, Shared<'d, D>);

#[derive(Clone)]
pub struct SetBg<'d, D: Drawable>(Color, Shared<'d, D>);

#[derive(Clone)]
pub struct SetClick<'d, D: Drawable>(Option<Affordance>, Shared<'d, D>);

#[derive(Clone)]
pub struct SetScroll<'d, D: Drawable>(Option<Affordance>, Shared<'d, D>);