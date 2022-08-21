use crate::shared::*;

use std::{rc::Rc, cell::RefCell};

pub(super) enum SharedMut<'d, D: Drawable> {
    Borrowed(Rc<RefCell<&'d mut D>>),
    Owned(Rc<RefCell<D>>),
}

impl<'d, D: Drawable> Clone for SharedMut<'d, D> {
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(arg0) => Self::Borrowed(arg0.clone()),
            Self::Owned(arg0) => Self::Owned(arg0.clone()),
        }
    }
}

impl<'d, D: Drawable> SharedMut<'d, D> {
    pub(super) fn wrap(arg: &'d mut D) -> Self {
        return SharedMut::Borrowed(Rc::new(RefCell::new(arg)))
    }

    pub(super) fn owned(arg: D) -> Self {
        return SharedMut::Owned(Rc::new(RefCell::new(arg)))
    }

    pub(super) fn borrow<T>(&self, f: impl FnOnce(&mut D) -> T) -> T {
        match self {
            SharedMut::Borrowed(x) => f(&mut x.borrow_mut()),
            SharedMut::Owned(x) => f(&mut x.borrow_mut())
        }
    }
}