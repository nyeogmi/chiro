use crate::{shared::*, Font};
use super::sharing::SharedMut;

mod reexport_cursor;
mod reexport_sharedmut;
mod trait_impl;

#[derive(Clone)]
pub struct Offset<'d, D: Drawable>(Zel, SharedMut<'d, D>);

#[derive(Clone)]
pub struct Clip<'d, D: Drawable>(ZelRect, SharedMut<'d, D>);

#[derive(Clone)]
pub struct SetFont<'d, D: Drawable>(Font, SharedMut<'d, D>);

#[derive(Clone)]
pub struct SetFg<'d, D: Drawable>(Color, SharedMut<'d, D>);

#[derive(Clone)]
pub struct SetBg<'d, D: Drawable>(Color, SharedMut<'d, D>);

#[derive(Clone)]
pub struct SetClick<'d, D: Drawable>(Option<Affordance>, SharedMut<'d, D>);

#[derive(Clone)]
pub struct SetScroll<'d, D: Drawable>(Option<Affordance>, SharedMut<'d, D>);