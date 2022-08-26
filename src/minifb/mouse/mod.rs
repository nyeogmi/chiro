mod drag;
mod scroll_wheel;
mod wiggle;

use std::collections::VecDeque;

use enum_map::EnumMap;
use euclid::{point2};
use minifb::{MouseButton as MinifbMouseButton, MouseMode, Window};

use self::{scroll_wheel::ScrollWheelMonitor, wiggle::WiggleMonitor};

use crate::{input::{MouseEvent, MouseButton}, shared::{Zel, Affordance, ZelSize}, screen::ZelData};

use drag::DragMonitor;


pub(crate) struct Mouse {
    drag: EnumMap<MouseButton, DragMonitor>,
    scroll_wheel: ScrollWheelMonitor,
    wiggle: WiggleMonitor,

    old: Option<State>,
    new: Option<State>,
    events: VecDeque<MouseEvent>,
}


#[derive(Clone, Copy)]
struct State {
    down: EnumMap<MouseButton, bool>,

    zel: Zel,
    click_selection: Option<Affordance>,
    scroll_selection: Option<Affordance>,
}


impl Mouse {
    pub fn new() -> Mouse {
        Mouse { 
            drag: enum_map::enum_map! {
                _ => DragMonitor::new(),
            },
            scroll_wheel: ScrollWheelMonitor::new(),
            wiggle: WiggleMonitor::new(),
            old: None, 
            new: None,
            events: VecDeque::new(),
        }
    }

    pub fn pop_event(&mut self) -> Option<MouseEvent> {
        self.events.pop_front()
    }

    // any_interactor: (normal, scroll)
    pub fn update(&mut self, size: ZelSize, window: &mut Window, new_tick: bool, get_zel: impl Fn(Zel) -> ZelData) {
        let current_state = Mouse::current_state(size, window, &get_zel);

        if let None = current_state {
            // don't bother generating events for now
            return;
        }

        self.old = self.new;
        self.new = current_state;

        use MouseEvent::*;

        match (self.old, self.new) {
            (None, None) => {}
            (None, Some(_)) => {}
            (Some(_), None) => {}
            (Some(old), Some(new)) => {
                if let Some((_, scroll_y)) = window.get_scroll_wheel() {
                    self.scroll_wheel.at(&mut self.events, new.zel, scroll_y, &get_zel)
                }

                for mb in MouseButton::ALL {
                    if new.down[mb] && !old.down[mb] {
                        self.events.push_back(Click {
                            mouse_button: mb, 
                            now: new.zel, 
                            now_click_selection: new.click_selection, 
                            now_scroll_selection: new.scroll_selection,
                        });
                        self.drag[mb].down(new.zel);
                    }

                    self.drag[mb].at(new.zel);

                    if !new.down[mb] && old.down[mb] {
                        self.events.push_back(Up {
                            mouse_button: mb, 
                            now: new.zel, 
                            now_click_selection: new.click_selection,
                            now_scroll_selection: new.scroll_selection,
                        });
                        self.drag[mb].up(&mut self.events, mb, &get_zel)
                    }

                    if new_tick {
                        self.drag[mb].post_events(&mut self.events, mb, &get_zel)
                    }
                }
                self.wiggle.at(new.zel);
                self.wiggle.post_events(&mut self.events, &get_zel)
            }
        }
    }

    // normal interactor, scroll interactor
    fn current_state(size: ZelSize, window: &mut Window, get_zel: &impl Fn(Zel) -> ZelData) -> Option<State> {
        // NYEO NOTE: The logic in minifb to compensate for DPI scaling is wrong.
        // This logic is correct, however.
        let mouse_pos = if let Some(mp) = window.get_unscaled_mouse_pos(MouseMode::Pass) { 
            mp 
        } else { return None };
        let overall_size = window.get_size();
        let mouse_x_ideal = ((mouse_pos.0 / overall_size.0 as f32) * size.width as f32) as i32;
        let mouse_y_ideal = ((mouse_pos.1 / overall_size.1 as f32) * size.height as f32) as i32;

        let zel_xy: Zel = point2(mouse_x_ideal as i32, mouse_y_ideal as i32);
        let click_selection: Option<Affordance>;
        let scroll_selection: Option<Affordance>;

        if mouse_x_ideal >= 0 && mouse_y_ideal >= 0 && mouse_x_ideal < size.width as i32 && mouse_y_ideal < size.height as i32 {
            let zel = get_zel(zel_xy);
            click_selection = zel.click;
            scroll_selection = zel.scroll;
        } else {
            click_selection = None;
            scroll_selection = None;
        }

        Some(State { 
            down: enum_map::enum_map![
                MouseButton::Left => window.get_mouse_down(MinifbMouseButton::Left),
                MouseButton::Right => window.get_mouse_down(MinifbMouseButton::Right),
            ],
            zel: zel_xy,
            click_selection,
            scroll_selection,
        })
    }

    pub(crate) fn selection_changed(&self) -> bool {
        return self.old.and_then(|n| n.click_selection) != self.new.and_then(|n| n.click_selection);
    }

    pub fn click_selection(&self) -> Option<Affordance> {
        self.new.and_then(|n| n.click_selection)
    }
}
// TODO: Scroll wheel?
// TODO: Drag events?