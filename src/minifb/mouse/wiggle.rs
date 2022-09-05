use std::collections::VecDeque;

use crate::{shared::*, input::MouseEvent, screen::ZelData};

#[derive(Clone, Copy)]
pub struct WiggleMonitor {
    old: Option<State>,
    event_to_send: Option<ToSend>, 
}

#[derive(Clone, Copy)]
struct ToSend {
    last: Zel,
    now: Zel,
}

#[derive(Clone, Copy)]
pub struct State {
    point: Zel,
}

impl WiggleMonitor {
    pub fn new() -> WiggleMonitor {
        WiggleMonitor {
            old: None,
            event_to_send: None,  // use this to rate-limit
        }
    }

    pub(crate) fn at(
        &mut self, 
        point: Zel, 
    ) {
        let new = State { point };
        let old = self.old.take();
        self.old.replace(new);
        let old = if let Some(old) = old { old } else { return };

        if let Some(e) = &mut self.event_to_send {
            e.now = new.point;
        } else {
            self.event_to_send = Some(ToSend { last: old.point, now: new.point });
        }
    }

    pub(crate) fn post_events(
        &mut self,
        events: &mut VecDeque<crate::input::MouseEvent>, 
        get_zel: &impl Fn(Zel) -> ZelData,
    ) {
        if let Some(ToSend { last, now }) = self.event_to_send.take() {
            let zel = get_zel(now);

            events.push_back(MouseEvent::Wiggle { last, now, now_click_selection: zel.click, now_scroll_selection: zel.scroll });
        }
    }
}
