use super::{fchar::FChar, fstring::FString};


pub trait Output {
}

pub trait ToFChar {
    fn to_fchar(&self) -> FChar;
}

pub trait ToFString: Sized {
    type FCharIterator: DoubleEndedIterator<Item = FChar>;

    fn to_fchars(self) -> Self::FCharIterator;
    fn to_fstring(self) -> FString {
        FString {characters: self.to_fchars().collect()}
    }
}

impl ToFString for FString {
    type FCharIterator = impl DoubleEndedIterator<Item = FChar>;

    fn to_fchars(self) -> Self::FCharIterator {
        self.characters.into_iter()
    }
}

impl<'a> ToFString for &'a FString {
    type FCharIterator = impl DoubleEndedIterator<Item = FChar>;

    fn to_fchars(self) -> Self::FCharIterator {
        self.characters.iter().cloned()
    }
}

impl<'a> ToFString for &'a str {
    type FCharIterator = impl DoubleEndedIterator<Item = FChar>;

    fn to_fchars(self) -> Self::FCharIterator {
        self.chars().map(FChar::from)
    }
}

impl ToFChar for FChar {
    fn to_fchar(&self) -> FChar { *self }
}

impl ToFChar for char {
    fn to_fchar(&self) -> FChar { FChar::from(*self) }
}