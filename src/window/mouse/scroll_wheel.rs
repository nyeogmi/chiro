use std::collections::VecDeque;

use crate::{aliases::*, input::MouseEvent};

pub struct ScrollWheelMonitor {
}

impl ScrollWheelMonitor {
    pub fn new() -> ScrollWheelMonitor {
        ScrollWheelMonitor { }
    }

    pub(crate) fn at(
        &mut self, 
        events: &mut VecDeque<MouseEvent>, 
        point: ZelPoint, 
        scroll_y: f32,
        affordance: &impl Fn(ZelPoint) -> Option<Affordance>,
    ) {
        // NOTE: Currently scroll_y is always divisible by 12
        events.push_back(MouseEvent::Scroll(-scroll_y / 12.0, point, affordance(point)));
    }
}
