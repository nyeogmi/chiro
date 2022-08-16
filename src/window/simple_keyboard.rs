use std::collections::VecDeque;

use enum_map::EnumMap;
use minifb::{Key as MinifbKey, Window};

use crate::input::{SimpleKeycode, SimpleKeyEvent};

pub(crate) struct SimpleKeyboard {
    keys_down: EnumMap<SimpleKeycode, bool>,
    events: VecDeque<SimpleKeyEvent>,
}

impl SimpleKeyboard {
    pub fn new() -> Self {
        SimpleKeyboard { keys_down: EnumMap::default(), events: VecDeque::new() }
    }

    pub fn update(&mut self, window: &mut Window) {
        // pressed keys
        if let Some(keys_down) = window.get_keys() {  
            let mut new_keys_down = EnumMap::default();

            for key in keys_down {
                if let Some(skey) = minifb_to_simple_keycode(key) {
                    new_keys_down[skey] = true;
                }
            }

            for (key, new_down) in new_keys_down.iter() {
                let old_down = self.keys_down[key];

                match (old_down, *new_down) {
                    (false, true) => { self.events.push_back(SimpleKeyEvent::Press(key)); }
                    (true, false) => { self.events.push_back(SimpleKeyEvent::Release(key)); }
                    (true, true) => {}
                    (false, false) => {}
                }
            }

            self.keys_down = new_keys_down;
        }
    }

    pub fn pop_event(&mut self) -> Option<SimpleKeyEvent> {
        self.events.pop_front()
    }
}

fn minifb_to_simple_keycode(key: MinifbKey) -> Option<SimpleKeycode> {
    use MinifbKey as M;
    use SimpleKeycode::*;

    Some(match key {
        M::Key0 => Key0, M::Key1 => Key1, M::Key2 => Key2, M::Key3 => Key3,
        M::Key4 => Key4, M::Key5 => Key5, M::Key6 => Key6, M::Key7 => Key7,
        M::Key8 => Key8, M::Key9 => Key9,

        M::NumPad0 => Key0, M::NumPad1 => Key1, M::NumPad2 => Key2,
        M::NumPad3 => Key3, M::NumPad4 => Key4, M::NumPad5 => Key5,
        M::NumPad6 => Key6, M::NumPad7 => Key7, M::NumPad8 => Key8,
        M::NumPad9 => Key9, 

        M::A => A, M::B => B, M::C => C, M::D => D, M::E => E, M::F => F,
        M::G => G, M::H => H, M::I => I, M::J => J, M::K => K, M::L => L,
        M::M => M, M::N => N, M::O => O, M::P => P, M::Q => Q, M::R => R,
        M::S => S, M::T => T, M::U => U, M::V => V, M::W => W, M::X => X,
        M::Y => Y, M::Z => Z,

        M::F1 => F1, M::F2 => F2, M::F3 => F3, M::F4 => F4, M::F5 => F5, 
        M::F6 => F6, M::F7 => F7, M::F8 => F8, M::F9 => F9, M::F10 => F10, 
        M::F11 => F11, M::F12 => F12, M::F13 => F13, M::F14 => F14, M::F15 => F15, 

        M::Down => Down, M::Left => Left, M::Right => Right, M::Up => Up,
        M::Apostrophe => Apostrophe, M::Backquote => Backquote,

        M::Backslash => Backslash, M::Comma => Comma, M::Equal => Equal,
        M::LeftBracket => LeftBracket, M::Minus => Minus, M::Period => Period,
        M::RightBracket => RightBracket, M::Semicolon => Semicolon,

        // we get backspaces specifically from text
        M::Slash => Slash, M::Backspace => return None, M::Delete => Delete,
        M::End => End, M::Enter => Enter,

        M::Escape => Escape,

        M::Home => Home, M::Insert => Insert, M::Menu => Menu,

        M::PageDown => PageDown, M::PageUp => PageUp,

        M::Pause => Pause, M::Space => Space, M::Tab => Tab,

        M::NumPadDot => Period, M::NumPadSlash => Slash,
        M::NumPadAsterisk => { return None }, M::NumPadMinus => Minus,
        M::NumPadPlus => { return None }, M::NumPadEnter => Enter,

        M::LeftShift | M::RightShift => Shift,
        M::LeftCtrl | M::RightCtrl => Control,
        M::LeftSuper | M::RightSuper => Super,
        M::LeftAlt | M::RightAlt => Alt,

        M::NumLock | M::CapsLock | M::ScrollLock |
        M::Unknown | M::Count =>
            return None
    })
}