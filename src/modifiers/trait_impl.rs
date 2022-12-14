use crate::{Drawable, screen::ZelData};
use super::*;

// == drawable implementation for modifiers
impl<'d, D:Drawable> Drawable for Offset<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&self) -> ZelRect {
        self.1.borrow(|d| d.bounds().translate(-self.0.to_vector()))
    }

    fn raw_touch(&mut self, zp: Zel, format: bool, modify: impl FnOnce(&mut ZelData)) {
        self.1.borrow(|d| d.raw_touch(zp.add_size(&self.0.to_vector().to_size()), format, modify))
    }

    fn deposit_supertile(&mut self, zp: Zel, tile: SuperTile) {
        self.1.borrow(|d| d.deposit_supertile(zp.add_size(&self.0.to_vector().to_size()), tile))
    }
}

impl<'d, D:Drawable> Drawable for Clip<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&self) -> ZelRect { self.0 }

    fn raw_touch(&mut self, zp: Zel, format: bool, modify: impl FnOnce(&mut ZelData)) {
        if !self.0.contains(zp) { return; }
        self.1.borrow(|d| d.raw_touch(zp, format, modify))
    }

    fn deposit_supertile(&mut self, zp: Zel, tile: SuperTile) {
        if !self.0.contains(zp) { return; }
        self.1.borrow(|d| d.deposit_supertile(zp, tile))
    }
}

impl<'d, D:Drawable> Drawable for SetFont<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.0 }

    fn bounds(&self) -> ZelRect { self.1.borrow(|d| d.bounds()) }

    fn raw_touch(&mut self, zp: Zel, format: bool, modify: impl FnOnce(&mut ZelData)) {
        self.1.borrow(|d| { d.raw_touch(zp, format, modify); } )
    }

    fn deposit_supertile(&mut self, zp: Zel, tile: SuperTile) {
        self.1.borrow(|d| { d.deposit_supertile(zp, tile)})
    }
}

impl<'d, D:Drawable> Drawable for SetFg<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&self) -> ZelRect { self.1.borrow(|d| d.bounds()) }

    fn raw_touch(&mut self, zp: Zel, format: bool, modify: impl FnOnce(&mut ZelData)) {
        self.1.borrow(|d| { d.raw_touch(zp, format, |z| { 
            if format { z.fg = self.0; } 
            modify(z);
        }); } )
    }

    fn deposit_supertile(&mut self, zp: Zel, tile: SuperTile) {
        self.1.borrow(|d| { d.deposit_supertile(zp, tile)})
    }
}

impl<'d, D:Drawable> Drawable for SetBg<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&self) -> ZelRect { self.1.borrow(|d| d.bounds()) }

    fn raw_touch(&mut self, zp: Zel, format: bool, modify: impl FnOnce(&mut ZelData)) {
        self.1.borrow(|d| { d.raw_touch(zp, format, |z| { 
            if format { z.bg = self.0 }
            modify(z);
        }); } )
    }

    fn deposit_supertile(&mut self, zp: Zel, tile: SuperTile) {
        self.1.borrow(|d| { d.deposit_supertile(zp, tile)})
    }
}

impl<'d, D:Drawable> Drawable for SetClick<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&self) -> ZelRect { self.1.borrow(|d| d.bounds()) }

    fn raw_touch(&mut self, zp: Zel, format: bool, modify: impl FnOnce(&mut ZelData)) {
        self.1.borrow(|d| { d.raw_touch(zp, format, |z| { 
            if format { z.click = self.0 }
            modify(z); 
        }); } )
    }

    fn deposit_supertile(&mut self, zp: Zel, tile: SuperTile) {
        self.1.borrow(|d| { d.deposit_supertile(zp, tile)})
    }
}

impl<'d, D:Drawable> Drawable for SetScroll<'d, D> {
    fn affordance(&mut self) -> Affordance { self.1.borrow(|d| d.affordance()) }
    fn get_font(&self) -> Font { self.1.borrow(|d| d.get_font()) }

    fn bounds(&self) -> ZelRect { self.1.borrow(|d| d.bounds()) }

    fn raw_touch(&mut self, zp: Zel, format: bool, modify: impl FnOnce(&mut ZelData)) {
        self.1.borrow(|d| { d.raw_touch(zp, format, |z| { 
            if format { z.scroll = self.0 }
            modify(z); 
        }); } )
    }

    fn deposit_supertile(&mut self, zp: Zel, tile: SuperTile) {
        self.1.borrow(|d| { d.deposit_supertile(zp, tile)})
    }
}