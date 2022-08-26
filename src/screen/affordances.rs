use crate::shared::*;

#[derive(Clone)]
pub(crate) struct Affordances {
    next: u64
}

impl Affordances {
    pub fn new() -> Self {
        Affordances { next: 0 }
    }

    pub fn generate(&mut self) -> Affordance {
        let aff = Affordance(self.next);
        self.next = self.next.wrapping_add(1);
        aff
    }
}