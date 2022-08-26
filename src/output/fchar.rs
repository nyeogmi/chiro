use crate::{shared::Affordance, screen::Zel, Color};

#[derive(Clone, Copy)]
pub struct FChar {
    pub character: Option<char>,
    pub formatting: Formatting,
}

impl FChar {
    pub fn empty() -> Self {
        FChar { character: None, formatting: Formatting::default() }
    }
}

impl From<char> for FChar {
    fn from(c: char) -> Self {
        FChar { character: Some(c), formatting: Formatting::default() }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Formatting {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub click: Option<Option<Affordance>>,
    pub scroll: Option<Option<Affordance>>,
}
impl Formatting {
    pub fn makes_changes(&self) -> bool {
        self.fg.is_some() || self.bg.is_some() || self.click.is_some() || self.scroll.is_some()
    }

    pub fn apply(&self, zel: &mut Zel) {
        if let Some(f) = self.fg { zel.fg = f; }
        if let Some(b) = self.bg { zel.bg = b; }
        if let Some(c) = self.click { zel.click = c; }
        if let Some(s) = self.scroll { zel.scroll = s; }
    }
}