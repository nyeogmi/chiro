// this probably belongs in Drawable, but the errors are aggressively terrible if the path is long
// so I've moved it up here

use crate::{shared::*, Font};
use crate::shared::Shared;

mod reexport_cursor;
mod trait_impl;

#[derive(Clone)]
pub struct Offset<'d, D: Drawable>(pub(crate) Zel, pub(crate) Shared<'d, D>);

#[derive(Clone)]
pub struct Clip<'d, D: Drawable>(pub(crate) ZelRect, pub(crate) Shared<'d, D>);

#[derive(Clone)]
pub struct SetFont<'d, D: Drawable>(pub(crate) Font, pub(crate) Shared<'d, D>);

#[derive(Clone)]
pub struct SetFg<'d, D: Drawable>(pub(crate) Color, pub(crate) Shared<'d, D>);

#[derive(Clone)]
pub struct SetBg<'d, D: Drawable>(pub(crate) Color, pub(crate) Shared<'d, D>);

#[derive(Clone)]
pub struct SetClick<'d, D: Drawable>(pub(crate) Option<Affordance>, pub(crate) Shared<'d, D>);

#[derive(Clone)]
pub struct SetScroll<'d, D: Drawable>(pub(crate) Option<Affordance>, pub(crate) Shared<'d, D>);