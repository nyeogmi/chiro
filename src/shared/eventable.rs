use crate::{Event, input::{Input, TypeEvent, Keystroke}};

pub trait Eventable {
    fn next_event(&mut self) -> Event;
    fn input(&self) -> &Input;

    fn next_tick(&mut self) -> Option<u64> {
        loop {
            match self.next_event() {
                Event::Exit => return None,
                Event::Tick(t) => return Some(t),
                Event::Mouse(_) => {},
                Event::Type(_) => {},
                Event::Press(_) => {},
            }
        }
    }

    fn next_keystroke(&mut self) -> Option<Keystroke> {
        loop {
            match self.next_event() {
                Event::Exit => return None,
                Event::Tick(_) => {},
                Event::Mouse(_) => {},
                Event::Type(TypeEvent::Press(evt)) => return Some(evt),
                Event::Type(_) => {},
                Event::Press(_) => {},
            }
        }
    }

    fn next_char(&mut self) -> Option<char> {
        loop {
            match self.next_event() {
                Event::Exit => return None,
                Event::Tick(_) => {},
                Event::Mouse(_) => {},
                Event::Type(TypeEvent::Type(c)) => return Some(c),
                Event::Type(_) => {},
                Event::Press(_) => {},
            }
        }
    }
}