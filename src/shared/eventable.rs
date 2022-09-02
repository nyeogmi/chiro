use crate::{Event, input::{Input, TypeEvent, TypeKey}};

pub type Chiro<T> = Result<T, ChiroError>;

pub enum ChiroError { Closed }

pub trait Eventable {
    fn next_event(&mut self) -> Event;  // returns the next event. ends with an endless stream of Exits
    fn input(&self) -> &Input;

    fn is_open(&self) -> bool;
    fn next_tick(&mut self) -> Chiro<u64> {
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

    fn next_keystroke(&mut self) -> Chiro<TypeKey> {
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

    fn next_char(&mut self) -> Chiro<char> {
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

// extension methods to Eventable trait which assume the program will terminate before a ChiroError::Closed is returned
pub trait SimpleIO {
    fn tick(&mut self) -> u64;
    fn keystroke(&mut self) -> TypeKey;
    fn char(&mut self) -> char;
}


impl <E: Eventable> SimpleIO for E {
    fn tick(&mut self) -> u64 {
        match self.next_tick() {
            Ok(n) => n,
            Err(ChiroError::Closed) => panic!("tick should have occurred or program should have terminated")
        }
    }

    fn keystroke(&mut self) -> TypeKey {
        match self.next_keystroke() {
            Ok(n) => n,
            Err(ChiroError::Closed) => panic!("keystroke should have occurred or program should have terminated")
        }
    }

    fn char(&mut self) -> char {
        match self.next_char() {
            Ok(n) => n,
            Err(ChiroError::Closed) => panic!("char should have occurred or program should have terminated")
        }
    }
}