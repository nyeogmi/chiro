use crate::{FString, shared::{ToColor, Affordance}};

impl FString {
    // these should have the same names as in Drawable's modifiers
    pub fn fg(self, color: impl ToColor) -> Self {
        let fg = color.to_color();
        self.map_fchars(|mut c| { c.formatting.fg = Some(fg); c } )
    }

    pub fn bg(self, color: impl ToColor) -> Self {
        let bg = color.to_color();
        self.map_fchars(|mut c| { c.formatting.bg = Some(bg); c } )
    }

    pub fn click(self, affordance: Affordance) -> Self {
        self.map_fchars(|mut c| { c.formatting.click = Some(Some(affordance)); c } )
    }

    pub fn scroll(self, affordance: Affordance) -> Self {
        self.map_fchars(|mut c| { c.formatting.scroll = Some(Some(affordance)); c } )
    }

    pub fn no_click(self) -> Self {
        self.map_fchars(|mut c| { c.formatting.click = Some(None); c } )
    }

    pub fn no_scroll(self) -> Self {
        self.map_fchars(|mut c| { c.formatting.scroll = Some(None); c } )
    }
}