use std::collections::VecDeque;

use crate::{shared::*, input::MouseEvent, screen::ZelData};

pub struct ScrollWheelMonitor {
}

impl ScrollWheelMonitor {
    pub fn new() -> ScrollWheelMonitor {
        ScrollWheelMonitor { }
    }

    pub(crate) fn at(
        &mut self, 
        events: &mut VecDeque<MouseEvent>, 
        point: Zel, 
        scroll_y: f32,
        get_zel: &impl Fn(Zel) -> ZelData,
    ) {
        // NOTE: Currently scroll_y is always divisible by 12
        events.push_back(MouseEvent::Scroll(-scroll_y / 12.0, point, get_zel(point).scroll));
    }
}
