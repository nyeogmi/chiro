use crate::{FString, shared::{ToColor, Affordance}, FChar};

impl FChar {
    pub fn fg(self, color: impl ToColor) -> Self {
        let fg = color.to_color();
        self.map_draw(|mut c| { c.formatting.fg = Some(fg); c })
    }

    pub fn bg(self, color: impl ToColor) -> Self {
        let bg = color.to_color();
        self.map_draw(|mut c| { c.formatting.bg = Some(bg); c })
    }

    pub fn click(self, affordance: Affordance) -> Self {
        self.map_draw(|mut c| { c.formatting.click = Some(Some(affordance)); c })
    }

    pub fn scroll(self, affordance: Affordance) -> Self {
        self.map_draw(|mut c| { c.formatting.scroll = Some(Some(affordance)); c })
    }

    pub fn no_click(self) -> Self {
        self.map_draw(|mut c| { c.formatting.click = Some(None); c })
    }

    pub fn no_scroll(self) -> Self {
        self.map_draw(|mut c| { c.formatting.scroll = Some(None); c })
    }
}

impl FString {
    // these should have the same names as in Drawable's modifiers
    pub fn fg(self, color: impl ToColor) -> Self {
        let fg = color.to_color();
        self.map_fchars(|c| c.fg(fg))
    }

    pub fn bg(self, color: impl ToColor) -> Self {
        let bg = color.to_color();
        self.map_fchars(|c| c.bg(bg))
    }

    pub fn click(self, affordance: Affordance) -> Self {
        self.map_fchars(|c| c.click(affordance))
    }

    pub fn scroll(self, affordance: Affordance) -> Self {
        self.map_fchars(|c| c.scroll(affordance))
    }

    pub fn no_click(self) -> Self {
        self.map_fchars(|c| c.no_click())
    }

    pub fn no_scroll(self) -> Self {
        self.map_fchars(|c| c.no_scroll())
    }
}