use enum_map::EnumMap;
use euclid::point2;

use crate::shared::*;

use super::{Event, MouseEvent, TypeEvent, MouseButton, PressKey, PressEvent};

pub struct Input {
    keyboard: Keyboard,
    mouse: Mouse,
}

impl Input {
    pub fn new() -> Input {
        Input { keyboard: Keyboard::new(), mouse: Mouse::new() }
    }

    pub(crate) fn on_event(&mut self, event: Event) {
        match event {
            Event::Exit => {}
            Event::Tick(_) => { 
                self.keyboard.on_tick();
                self.mouse.on_tick();
            }
            Event::Press(k) => { self.keyboard.on_simple_event(k) }
            Event::Type(k) => { self.keyboard.on_event(k) }
            Event::Mouse(m) => { self.mouse.on_event(m) }
        }
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }
}

pub struct Keyboard {
    typed_chars: String,
    is_down: EnumMap<PressKey, bool>,
    is_pressed: EnumMap<PressKey, bool>,
    is_released: EnumMap<PressKey, bool>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            typed_chars: String::new(),
            is_down: EnumMap::default(),
            is_pressed: EnumMap::default(),
            is_released: EnumMap::default(),
        }
    }

    fn on_tick(&mut self) {
        self.typed_chars.clear();
        self.is_pressed = EnumMap::default();
        self.is_released = EnumMap::default();
    }

    fn on_event(&mut self, k: TypeEvent) {
        match k {
            TypeEvent::Press(_) => { }
            TypeEvent::Release(_) => { }
            TypeEvent::Type(c) => {
                self.typed_chars.push(c)
            },
        }
    }

    fn on_simple_event(&mut self, k: PressEvent) {
        match k {
            PressEvent::Press(kc) => {
                self.is_down[kc] = true;
                self.is_pressed[kc] = true;
            }
            PressEvent::Release(kc) => {
                self.is_down[kc] = false;
                self.is_released[kc] = true;
            }
        }
    }
}

pub struct Mouse {
    click_selection: Option<Affordance>,
    scroll_selection: Option<Affordance>,

    position: ZelPointI,
    scroll: f32,

    is_down: EnumMap<MouseButton, bool>,
    is_pressed: EnumMap<MouseButton, bool>,
    is_released: EnumMap<MouseButton, bool>,

    drag: EnumMap<MouseButton, Option<Drag>>,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            click_selection: None,
            scroll_selection: None,

            position: point2(0, 0),
            scroll: 0.0,

            is_down: EnumMap::default(),
            is_pressed: EnumMap::default(),
            is_released: EnumMap::default(),

            drag: EnumMap::default(),
        }
    }

    fn on_tick(&mut self) {
        self.scroll = 0.0;

        self.is_pressed = EnumMap::default();
        self.is_released = EnumMap::default();
    }

    fn on_event(&mut self, m: MouseEvent) {
        match m {
            MouseEvent::Click { mouse_button: mb, .. } => {
                self.is_pressed[mb] = true;
                self.is_down[mb] = true;
            }
            MouseEvent::Up { mouse_button: mb, .. } => {
                self.is_down[mb] = false;
                self.is_released[mb] = true;
                self.drag[mb] = None;
            }
            MouseEvent::Drag { mouse_button: mb, start, last, now, now_click_selection: _, now_scroll_selection: _} => {
                self.drag[mb] = Some(Drag { start, last, now })
            }
            MouseEvent::Wiggle { last: _, now, now_click_selection, now_scroll_selection } => {
                self.click_selection = now_click_selection;
                self.scroll_selection = now_scroll_selection;
                self.position = now;
            }
            MouseEvent::Scroll(amt, _, _) => {
                self.scroll += amt;
            }
        }
    }

    pub fn is_click_over(&self, affordance: Affordance) -> bool { self.click_selection == Some(affordance) }
    pub fn is_scroll_over(&self, affordance: Affordance) -> bool { self.scroll_selection == Some(affordance) }
    pub fn is_pressed(&self, button: MouseButton) -> bool { self.is_pressed[button] }
    pub fn is_released(&self, button: MouseButton) -> bool { self.is_pressed[button] }
    pub fn is_down(&self, button: MouseButton) -> bool { self.is_pressed[button] }

    pub fn left_clicked(&self, affordance: Affordance) -> bool { self.is_pressed(MouseButton::Left) && self.is_click_over(affordance) }
    pub fn right_clicked(&self, affordance: Affordance) -> bool { self.is_pressed(MouseButton::Right) && self.is_click_over(affordance) }
    pub fn scrolled_on(&self, affordance: Affordance) -> Option<f32> {
        if self.scroll == 0.0 || !self.is_scroll_over(affordance) { return None }
        return Some(self.scroll);
    }
}

pub struct Drag {
    pub start: ZelPointI,
    pub last: ZelPointI,
    pub now: ZelPointI,
}