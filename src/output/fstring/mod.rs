mod formatting;

use std::{ops::Add, fmt::Display};

use super::{fchar::{FChar, FCharDraw}, interface::ToFString};

#[derive(Clone, Default, Debug)]
pub struct FString {
    pub(super) characters: Vec<FChar>,
}

impl From<&str> for FString {
    fn from(string: &str) -> Self {
        FString { characters: string.chars().map(FChar::from).collect()}
    }
}

impl FString {
    pub fn with_capacity(capacity: usize) -> Self {
        FString { characters: Vec::with_capacity(capacity) }
    }
    
    pub fn push<'a>(&mut self, s: impl ToFString) {
        self.characters.extend(s.to_fchars());
    }

    pub fn truncate(&mut self, new_len: usize) {
        self.characters.truncate(new_len)
    }

    pub fn pop(&mut self) -> Option<FChar> {
        self.characters.pop()
    }

    pub fn remove(&mut self, idx: usize) -> Option<FChar> {
        if idx >= self.characters.len() {
            return None
        }
        return Some(self.characters.remove(idx))
    }

    pub fn retain(&mut self, mut f: impl FnMut(FChar) -> bool) {
        self.characters.retain(|x| f(*x))
    }
    
    pub fn insert(&mut self, idx: usize, s: impl ToFString) {  
        // non-panicking insert
        // TODO: Get rid of? This is an API change
        self.characters.splice(idx..idx, s.to_fchars());
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<FChar> { &mut self.characters }
    pub fn as_slice(&self) -> &[FChar] { &self.characters }

    pub fn len(&self) -> usize { self.characters.len() }
    pub fn is_empty(&self) -> bool { self.characters.is_empty()}

    pub fn split_off(&mut self, ix: usize) -> Option<FString> {
        if ix > self.characters.len() { return None }
        let characters = self.characters.split_off(ix);
        Some(FString { characters })
    }

    pub fn clear(&mut self) {  self.characters.clear() }

    // TODO: iterators?
    // TODO: drain, replace_range

    pub fn get(&self, ix: usize) -> Option<FChar> {
        self.characters.get(ix).map(|x| *x)
    }

    pub fn get_mut(&mut self, ix: usize) -> Option<&mut FChar> {
        self.characters.get_mut(ix)
    }

    /*
    pub fn starts_with(&mut self, s: impl ToFString) -> bool {
        self.characters.iter().zip(s.to_fchars()).all(|(x, y)| x.character == y.character)
    }

    pub fn ends_with(&mut self, s: impl ToFString) -> bool {
        self.characters.iter().rev().zip(s.to_fchars().rev()).all(|(x, y)| x.character == y.character)
    }
    */

    pub fn map_fchars(mut self, mut f: impl FnMut(FChar) -> FChar) -> FString {
        for i in self.characters.iter_mut() { *i = f(*i); }
        self
    }

    pub fn map_fchars_draw(mut self, mut f: impl FnMut(FCharDraw) -> FCharDraw) -> FString {
        for i in self.characters.iter_mut() { 
            i.map_draw(&mut f);
        }
        self
    }

    pub fn map_chars(self, mut f: impl FnMut(char) -> char) -> FString {
        self.map_fchars_draw(|mut fcd| { 
            fcd.character = f(fcd.character);
            fcd
        })
    }
    // TODO: impl trim() if needed
    pub fn to_ascii_lowercase(self) -> FString {
        self.map_chars(|x| x.to_ascii_lowercase())
    }

    pub fn to_ascii_uppercase(self) -> FString {
        self.map_chars(|x| x.to_ascii_uppercase())
    }

    pub fn repeat(&self, n: usize) -> FString {
        FString { characters: self.characters.repeat(n) }
    }
}

impl Add<FString> for FString {
    type Output = FString;

    fn add(self, rhs: Self) -> Self::Output {
        FString { characters: self.characters.iter().chain(rhs.characters.iter()).cloned().collect() }
    }
}

impl Add<FChar> for FString {
    type Output = FString;

    fn add(self, rhs: FChar) -> Self::Output {
        self + rhs.to_fstring()
    }
}

impl Add<FString> for FChar {
    type Output = FString;

    fn add(self, rhs: FString) -> Self::Output {
        self.to_fstring() + rhs
    }
}

impl Display for FString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let simplified = String::from_iter(self.characters.iter().map(|i| 
            match i {
                FChar::Empty => ' ',
                FChar::Draw(fcd) => fcd.character,
                FChar::Newline => '\n',
            }
        ));
        simplified.fmt(f)
    }
}

impl <'a> Extend<&'a FChar> for FString {
    fn extend<T: IntoIterator<Item = &'a FChar>>(&mut self, iter: T) {
        self.characters.extend(iter)
    }
}

impl Extend<FChar> for FString {
    fn extend<T: IntoIterator<Item = FChar>>(&mut self, iter: T) {
        self.characters.extend(iter)
    }
}

impl <'a> Extend<&'a FString> for FString {
    fn extend<T: IntoIterator<Item = &'a FString>>(&mut self, iter: T) {
        for i in iter {
            self.characters.extend(&i.characters)
        }
    }
}

impl Extend<FString> for FString {
    fn extend<T: IntoIterator<Item = FString>>(&mut self, iter: T) {
        for i in iter {
            self.characters.extend(&i.characters)
        }
    }
}