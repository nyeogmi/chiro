use crate::{Event, input::{Input, TypeEvent, TypeKey}};

pub type ChiroResult<T> = Result<T, ChiroError>;

pub enum ChiroError { Closed }

pub trait Eventable {
    fn next_event(&mut self) -> Event;  // returns the next event. ends with an endless stream of Exits
    fn input(&self) -> &Input;

    fn is_open(&self) -> bool;
    fn next_tick(&mut self) -> ChiroResult<u64> {
        loop {
            match self.next_event() {
                Event::Exit => { return Err(ChiroError::Closed) }
                Event::Tick(t) => { return Ok(t) }
                Event::Mouse(_) => {},
                Event::Type(_) => {},
                Event::Press(_) => {},
            }
        }
    }

    fn next_keystroke(&mut self) -> ChiroResult<TypeKey> {
        loop {
            match self.next_event() {
                Event::Exit => { return Err(ChiroError::Closed) },
                Event::Tick(_) => {},
                Event::Mouse(_) => {},
                Event::Type(TypeEvent::Down(evt)) => { return Ok(evt) }
                Event::Type(_) => {},
                Event::Press(_) => {},
            }
        }
    }

    fn next_char(&mut self) -> ChiroResult<char> {
        loop {
            match self.next_event() {
                Event::Exit => { return Err(ChiroError::Closed) },
                Event::Tick(_) => {},
                Event::Mouse(_) => {},
                Event::Type(TypeEvent::Type(c)) => return Ok(c),
                Event::Type(_) => {},
                Event::Press(_) => {},
            }
        }
    }
}