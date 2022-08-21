use crate::{Drawable, screen::Zel};
use super::*;

// == drawable implementation for modifiers
impl<'d, D:Drawable> Drawable for Offset<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&mut self) -> ZelRectI {
        self.1.borrow(|d| d.bounds().translate(-self.0.to_vector()))
    }

    fn raw_view(&self, zp: ZelPointI) -> Zel {
        self.1.borrow(|d| d.raw_view(zp.add_size(&self.0.to_vector().to_size())))
    }

    fn raw_touch(&mut self, zp: ZelPointI, format: bool, modify: impl FnOnce(&mut Zel)) {
        self.1.borrow(|d| d.raw_touch(zp.add_size(&self.0.to_vector().to_size()), format, modify))
    }
}

impl<'d, D:Drawable> Drawable for Clip<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&mut self) -> ZelRectI { self.0 }

    fn raw_view(&self, zp: ZelPointI) -> Zel {
        if !self.0.contains(zp) { return Zel::default() }
        return self.1.borrow(|d| d.raw_view(zp))
    }

    fn raw_touch(&mut self, zp: ZelPointI, format: bool, modify: impl FnOnce(&mut Zel)) {
        if !self.0.contains(zp) { return; }
        self.1.borrow(|d| d.raw_touch(zp, format, modify))
    }
}

impl<'d, D:Drawable> Drawable for SetFont<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.0 }

    fn bounds(&mut self) -> ZelRectI { self.1.borrow(|d| d.bounds()) }
    fn raw_view(&self, zp: ZelPointI) -> Zel { self.1.borrow(|d| d.raw_view(zp)) }

    fn raw_touch(&mut self, zp: ZelPointI, format: bool, modify: impl FnOnce(&mut Zel)) {
        self.1.borrow(|d| { d.raw_touch(zp, format, modify); } )
    }
}

impl<'d, D:Drawable> Drawable for SetFg<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&mut self) -> ZelRectI { self.1.borrow(|d| d.bounds()) }
    fn raw_view(&self, zp: ZelPointI) -> Zel { self.1.borrow(|d| d.raw_view(zp)) }

    fn raw_touch(&mut self, zp: ZelPointI, format: bool, modify: impl FnOnce(&mut Zel)) {
        self.1.borrow(|d| { d.raw_touch(zp, format, |z| { 
            modify(z);
            if format { z.fg = self.0 } });
        } )
    }
}

impl<'d, D:Drawable> Drawable for SetBg<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&mut self) -> ZelRectI { self.1.borrow(|d| d.bounds()) }
    fn raw_view(&self, zp: ZelPointI) -> Zel { self.1.borrow(|d| d.raw_view(zp)) }

    fn raw_touch(&mut self, zp: ZelPointI, format: bool, modify: impl FnOnce(&mut Zel)) {
        self.1.borrow(|d| { d.raw_touch(zp, format, |z| { 
            modify(z);
            if format { z.bg = self.0 }
        }); } )
    }
}

impl<'d, D:Drawable> Drawable for SetClick<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&mut self) -> ZelRectI { self.1.borrow(|d| d.bounds()) }
    fn raw_view(&self, zp: ZelPointI) -> Zel { self.1.borrow(|d| d.raw_view(zp)) }

    fn raw_touch(&mut self, zp: ZelPointI, format: bool, modify: impl FnOnce(&mut Zel)) {
        self.1.borrow(|d| { d.raw_touch(zp, format, |z| { 
            modify(z); 
            if format { z.click = self.0 }
        }); } )
    }
}

impl<'d, D:Drawable> Drawable for SetScroll<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&mut self) -> ZelRectI { self.1.borrow(|d| d.bounds()) }
    fn raw_view(&self, zp: ZelPointI) -> Zel { self.1.borrow(|d| d.raw_view(zp)) }

    fn raw_touch(&mut self, zp: ZelPointI, format: bool, modify: impl FnOnce(&mut Zel)) {
        self.1.borrow(|d| { d.raw_touch(zp, format, |z| { 
            modify(z); 
            if format { z.scroll = self.0 }
        }); } )
    }
}