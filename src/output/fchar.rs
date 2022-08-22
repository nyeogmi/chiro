use crate::shared::Affordance;

#[derive(Clone, Copy)]
pub struct FChar {
    pub character: char,
    pub formatting: Formatting,
}

#[derive(Clone, Copy, Default)]
pub struct Formatting {
    pub fg: Option<u32>,
    pub bg: Option<u32>,
    pub click: Option<Affordance>,
    pub scroll: Option<Affordance>,
}