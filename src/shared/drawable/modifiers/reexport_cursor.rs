use crate::{shared::*, Font};
use super::At;


impl<'a, D: Drawable> At<'a, D> {
    // == reexport all modifiers ==
    // these don't require &mut, but for type-consistency with Drawable, let's require it
    // forcing a manual clone()

    pub fn offset(&self, xy: impl ToZelPointI) -> At<'a, Offset<D>> {
        let xy = xy.to_zeli();
        self._internally(|x| x.offset(xy))
    }

    pub fn clip(&self, xy0: impl ToZelPointI, xy1: impl ToZelPointI) -> At<'a, Clip<D>> {
        let xy0 = xy0.to_zeli();
        let xy1 = xy1.to_zeli();
        self._internally(|x| x.clip(xy0, xy1))
    }

    pub fn font(&self, font: Font) -> At<'a, SetFont<D>> { 
        self._internally(|x| x.font(font))
    }

    pub fn fg(&self, color: impl ToColor) -> At<'a, SetFg<D>> { 
        let color = color.to_color();
        self._internally(|x| x.fg(color))
    }

    pub fn bg(&self, color: impl ToColor) -> At<'a, SetBg<D>> { 
        let color = color.to_color();
        self._internally(|x| x.bg(color))
    }

    pub fn click(&self, affordance: Affordance) -> At<'a, SetClick<D>> { 
        self._internally(|x| x.click(affordance))
    }

    pub fn scroll(&self, affordance: Affordance) -> At<'a, SetScroll<D>> { 
        self._internally(|x| x.scroll(affordance))
    }

    pub fn no_click(&self) -> At<'a, SetClick<D>> { 
        self._internally(|x| x.no_click())
    }

    pub fn no_scroll(&self) -> At<'a, SetScroll<D>> { 
        self._internally(|x| x.no_scroll())
    }

    pub fn offset_i(&self, xy: (i32, i32)) -> At<'a, Offset<D>> { 
        self._internally(|x| x.offset_i(xy))
    }

    pub fn clip_i(&self, xy0: (i32, i32), xy1: (i32, i32)) -> At<'a, Clip<D>> { 
        self._internally(|x| x.clip_i(xy0, xy1))
    }
}