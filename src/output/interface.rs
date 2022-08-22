use std::{slice::Iter, iter::Cloned};

use super::{fchar::FChar, fstring::FString};


pub trait Output {
}

pub trait ToFString: Sized {
    type FCharIterator: DoubleEndedIterator<Item = FChar>;

    fn to_fchars(self) -> Self::FCharIterator;
    fn to_fstring(self) -> FString {
        FString {characters: self.to_fchars().collect()}
    }
}

impl<'a> ToFString for &'a FString {
    type FCharIterator = Cloned<Iter<'a, FChar>>;

    fn to_fchars(self) -> Self::FCharIterator {
        self.characters.iter().cloned()
    }
}