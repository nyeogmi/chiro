use enum_map::Enum;

use crate::aliases::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    Tick(u64),
    Mouse(MouseEvent),
    Keyboard(KeyEvent),
    SimpleKeyboard(SimpleKeyEvent),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MouseEvent {
    Click(MouseButton, ZelPoint, Option<Affordance>),
    Up(MouseButton, ZelPoint, Option<Affordance>),
    Drag { 
        mouse_button: MouseButton, 
        start: ZelPoint, 
        last: ZelPoint,
        now: ZelPoint,
        now_selection: Option<Affordance>,
    },
    Wiggle { 
        last: ZelPoint,
        now: ZelPoint,
        now_selection: Option<Affordance>,
    },
    Scroll(f32, ZelPoint, Option<Affordance>),
    // wheel?
}

#[derive(Clone, Copy, Debug, Enum, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MouseButton {
    Left, Right
}

impl MouseButton {
    pub const ALL: [MouseButton; 2] = [MouseButton::Left, MouseButton::Right];
}

// TODO: Add an "is_accept()" method that returns true for enter and space
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyEvent {
    Press(KeyCombo),
    Release(KeyCombo),
    Type(char),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyCombo {
    pub code: Keycode,
    pub shift: bool,
    pub control: bool,
}

#[derive(Debug, Enum, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum Keycode {
    // Unashamedly inspired by a similar enum from minifb
    Key0 = 0, Key1 = 1, Key2 = 2, Key3 = 3, Key4 = 4,
    Key5 = 5, Key6 = 6, Key7 = 7, Key8 = 8, Key9 = 9,

    A = 10, B = 11, C = 12, D = 13, E = 14, F = 15,
    G = 16, H = 17, I = 18, J = 19, K = 20, L = 21,
    M = 22, N = 23, O = 24, P = 25, Q = 26, R = 27,
    S = 28, T = 29, U = 30, V = 31, W = 32, X = 33,
    Y = 34, Z = 35,

    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15,

    Down, Left, Right, Up,
    Apostrophe, Backquote,

    Backslash, Comma, Equal, LeftBracket,
    Minus, Period, RightBracket, Semicolon,
    Slash, Backspace, Delete, End, Enter,

    Escape,
    Home, Insert, Menu,
    PageDown, PageUp,
    Pause, 
    
    Space, Tab,

    // TODO: Shift punctuation
    Tilde,
    Exclamation, At, Pound, Dollar, Percent, Caret, Ampersand, Asterisk,
    LeftParen, RightParen, Underscore, Plus, LeftBrace, RightBrace,
    Pipe, Colon, DoubleQuote,
    LessThan, GreaterThan, QuestionMark,

    // don't include Lock, Shift, Alt, Super, and Ctrl -- terminals don't respond to 
    // these by themselves

    // don't expose NumPad keys separately: terminals don't know the difference
    // and doing so encourages developers to make UIs that won't work on most laptops
}


impl KeyEvent {
    pub fn alter_combo(&mut self, alter: impl FnOnce(&mut KeyCombo)) {
        match self {
            KeyEvent::Press(k) => alter(k),
            KeyEvent::Release(k) => alter(k),
            KeyEvent::Type(_) => {},
        }
    }

    pub fn get_combo(&self) -> Option<KeyCombo> {
        match self {
            KeyEvent::Press(k) => Some(*k),
            KeyEvent::Release(k) => Some(*k),
            KeyEvent::Type(_) => None,
        }
    }
    
    pub fn is_down(&self) -> bool {
        match self {
            KeyEvent::Press(_) => true,
            KeyEvent::Release(_) => false,
            KeyEvent::Type(_) => true,
        }
    }

    pub fn is_up(&self) -> bool {
        return !self.is_down()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleKeyEvent {
    Press(SimpleKeycode),
    Release(SimpleKeycode),
}

#[derive(Debug, Enum, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SimpleKeycode {
    // Unashamedly inspired by a similar enum from minifb
    // Removes some keys that terminals would understand but which a game wouldn't
    Key0 = 0, Key1 = 1, Key2 = 2, Key3 = 3, Key4 = 4,
    Key5 = 5, Key6 = 6, Key7 = 7, Key8 = 8, Key9 = 9,

    A = 10, B = 11, C = 12, D = 13, E = 14, F = 15,
    G = 16, H = 17, I = 18, J = 19, K = 20, L = 21,
    M = 22, N = 23, O = 24, P = 25, Q = 26, R = 27,
    S = 28, T = 29, U = 30, V = 31, W = 32, X = 33,
    Y = 34, Z = 35,

    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15,

    Down, Left, Right, Up,
    Apostrophe, Backquote,

    Backslash, Comma, Equal, LeftBracket,
    Minus, Period, RightBracket, Semicolon,
    Slash, Backspace, Delete, End, Enter,

    Escape,
    Home, Insert, Menu,
    PageDown, PageUp,
    Pause, 
    
    Space, Tab,

    Shift, Control, Super, Alt,
}