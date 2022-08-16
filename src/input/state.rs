use enum_map::EnumMap;
use euclid::point2;

use crate::aliases::*;

use super::{Event, Keycode, MouseEvent, KeyEvent, MouseButton, SimpleKeycode, SimpleKeyEvent, KeyCombo};

pub struct Input {
    keyboard: Keyboard,
    mouse: Mouse,
}

impl Input {
    pub fn new() -> Input {
        Input { keyboard: Keyboard::new(), mouse: Mouse::new() }
    }

    pub fn on_event(&mut self, event: Event) {
        match event {
            Event::Tick(_) => { 
                self.keyboard.on_tick();
                self.mouse.on_tick();
            }
            Event::SimpleKeyboard(k) => { self.keyboard.on_simple_event(k) }
            Event::Keyboard(k) => { self.keyboard.on_event(k) }
            Event::Mouse(m) => { self.mouse.on_event(m) }
        }
    }
}

pub struct Keyboard {
    typed_chars: String,
    is_down: EnumMap<SimpleKeycode, bool>,
    is_pressed: EnumMap<SimpleKeycode, bool>,
    is_released: EnumMap<SimpleKeycode, bool>,
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

    fn on_event(&mut self, k: KeyEvent) {
        match k {
            // TODO: These cases existed in chiropterm 1
            // Do we need them?
            KeyEvent::Press(KeyCombo { shift: false, control: false, code: Keycode::Backspace }) => { 
                self.on_simple_event(SimpleKeyEvent::Press(SimpleKeycode::Backspace))
            }
            KeyEvent::Release(KeyCombo { shift: false, control: false, code: Keycode::Backspace }) => { 
                self.on_simple_event(SimpleKeyEvent::Release(SimpleKeycode::Backspace))
            }
            KeyEvent::Press(_) => { }
            KeyEvent::Release(_) => {}
            KeyEvent::Type(c) => {
                self.typed_chars.push(c)
            },
        }
    }

    fn on_simple_event(&mut self, k: SimpleKeyEvent) {
        match k {
            SimpleKeyEvent::Press(kc) => {
                self.is_down[kc] = true;
                self.is_pressed[kc] = true;
            }
            SimpleKeyEvent::Release(kc) => {
                self.is_down[kc] = false;
                self.is_released[kc] = true;
            }
        }
    }
}

pub struct Mouse {
    selection: Option<Affordance>,
    position: ZelPoint,
    scroll: f32,

    is_down: EnumMap<MouseButton, bool>,
    is_pressed: EnumMap<MouseButton, bool>,
    is_released: EnumMap<MouseButton, bool>,

    drag: EnumMap<MouseButton, Option<Drag>>,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            selection: None,
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
            MouseEvent::Click(mb, _, _) => {
                self.is_pressed[mb] = true;
                self.is_down[mb] = true;
            }
            MouseEvent::Up(mb, _, _) => {
                self.is_down[mb] = false;
                self.is_released[mb] = true;
                self.drag[mb] = None;
            }
            MouseEvent::Drag { mouse_button: mb, start, last, now, now_selection: _} => {
                self.drag[mb] = Some(Drag { start, last, now })
            }
            MouseEvent::Wiggle { last: _, now, now_selection } => {
                self.selection = now_selection;
                self.position = now;
            }
            MouseEvent::Scroll(amt, _, _) => {
                self.scroll += amt;
            }
        }
    }
}

pub struct Drag {
    pub start: ZelPoint,
    pub last: ZelPoint,
    pub now: ZelPoint,
}