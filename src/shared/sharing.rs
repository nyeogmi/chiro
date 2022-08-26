use crate::shared::*;

use std::{rc::Rc, cell::{RefCell, RefMut}, borrow::BorrowMut, mem};

pub(crate) struct Shared<'d, D: Drawable+'d>(RefCell<Backing<'d, D>>);

enum Backing<'d, D: Drawable> {
    Single(&'d mut D),
    Borrowed(Rc<RefCell<&'d mut D>>),
    Owned(Rc<RefCell<D>>),
    Poisoned,
}

impl<'d, D: Drawable> Clone for Shared<'d, D> {
    fn clone(&self) -> Self {
        let out = {
            let mut here: RefMut<Backing<'d, D>> = self.0.borrow_mut();
            let mut poison: Backing<'d, D> = Backing::Poisoned;
            mem::swap(&mut *here, &mut poison);
            let (mut l, r) = poison.split();
            mem::swap(&mut *here, &mut l);
            Shared(RefCell::new(r))
        };
        assert!(!self.0.borrow().poisoned());
        assert!(!out.0.borrow().poisoned());
        return out
    }
}

impl<'d, D: Drawable> Backing<'d, D> {
    #[inline(always)]
    fn split(self) -> (Backing<'d, D>, Backing<'d, D>) {
        match self {
            Backing::Single(d) => {
                let rc = Rc::new(RefCell::new(d));
                (Backing::Borrowed(rc.clone()), Backing::Borrowed(rc))
            }
            Backing::Borrowed(rc) => {
                (Backing::Borrowed(rc.clone()), Backing::Borrowed(rc))
            }
            Backing::Owned(rc) => {
                (Backing::Owned(rc.clone()), Backing::Owned(rc))
            }
            Backing::Poisoned => unreachable!()
        }
    }

    #[inline(always)]
    fn poisoned(&self) -> bool {
        if let Backing::Poisoned = self { return true }
        false
    }
}

impl<'d, D: Drawable> Shared<'d, D> {
    pub fn wrap(arg: &'d mut D) -> Self {
        return Shared(RefCell::new(Backing::Single(arg)))
    }

    pub fn owned(arg: D) -> Self {
        return Shared(RefCell::new(Backing::Owned(Rc::new(RefCell::new(arg)))))
    }

    pub fn borrow<T>(&self, f: impl FnOnce(&mut D) -> T) -> T {
        let mut x: RefMut<Backing<'d, D>> = self.0.borrow_mut();
        match &mut *x {
            Backing::Single(x) => f(x),
            Backing::Borrowed(x) => {
                let mut d: RefMut<&mut D> = (**x).borrow_mut();
                f(d.borrow_mut())
            }
            Backing::Owned(x) => {
                let mut d: RefMut<D> = (**x).borrow_mut();
                f(&mut d.borrow_mut())
            }
            Backing::Poisoned => unreachable!()
        }
    }
}