use std::collections::VecDeque;

use crate::{shared::*, screen::Zel};
use super::{MouseButton, MouseEvent};

#[derive(Clone, Copy)]
pub struct DragMonitor {
    start: Option<State>,
    old: Option<State>,
    event_to_send: Option<ToSend>, 
}

#[derive(Clone, Copy)]
struct ToSend {
    start: ZelPointI,
    last: ZelPointI,
    now: ZelPointI,
}

#[derive(Clone, Copy)]
pub struct State {
    point: ZelPointI,
}

impl DragMonitor {
    pub fn new() -> DragMonitor {
        DragMonitor {
            start: None,
            old: None,
            event_to_send: None,  // use this to rate-limit
        }
    }

    pub(crate) fn down(&mut self, point: ZelPointI) {
        self.start = Some(State { point });
        self.old = self.start
    }

    pub(crate) fn at(
        &mut self, 
        point: ZelPointI, 
    ) {
        if self.start.is_none() { return; }
        let start = self.start.unwrap();
        let old = self.old.unwrap();  // set when start is set
        let new = State { point };

        if old.point == new.point { return }

        if let Some(e) = &mut self.event_to_send {
            e.now = new.point;
        } else {
            self.event_to_send = Some(ToSend { start: start.point, last: old.point, now: new.point });
        }
        self.old = Some(new);
    }

    pub(crate) fn post_events(
        &mut self,
        events: &mut VecDeque<crate::input::MouseEvent>, 
        mouse_button: MouseButton,
        get_zel: &impl Fn(ZelPointI) -> Zel,
    ) {
        if let Some(ToSend { start, last, now }) = self.event_to_send.take() {
            let zel = get_zel(now);

            events.push_back(MouseEvent::Drag {
                mouse_button,
                start,
                last,
                now,
                now_click_selection: zel.click,
                now_scroll_selection: zel.scroll,
            });
        }
    }

    pub(crate) fn up(
        &mut self,
        events: &mut VecDeque<crate::input::MouseEvent>,
        mouse_button: MouseButton,
        get_zel: &impl Fn(ZelPointI) -> Zel,
    ) {
        self.post_events(events, mouse_button, get_zel);

        self.event_to_send = None;
        self.start = None;
        self.old = None;
    }
}
