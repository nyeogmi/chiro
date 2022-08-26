use super::{fstring::FString, fchar::{FChar, Formatting, FCharDraw}};


const NONE_PLACEHOLDER: char = '\u{0F0000}';  // a private use character
const CHAR10_PLACEHOLDER: char = '\u{0F0001}';  // a private use character


fn to_output_placeholder(fc: FChar) -> char {
    match fc {
        FChar::Empty => NONE_PLACEHOLDER,
        FChar::Draw(fcd) => 
            if let '\n' = fcd.character { CHAR10_PLACEHOLDER }
            else { fcd.character },
        FChar::Newline => '\n',
    }
}

fn from_output_placeholder(c: char, formatting: Formatting) -> FChar {
    match c {
        NONE_PLACEHOLDER => FChar::Empty,
        CHAR10_PLACEHOLDER => FChar::Draw(FCharDraw { character: c, formatting }),
        '\n' => FChar::Newline,
        c => FChar::Draw(FCharDraw { character: c, formatting }),
    }
}

fn update_formatting(formatting: &mut Formatting, src: FChar) {
    if let FChar::Draw(fcd) = src {
        *formatting = fcd.formatting;
    }
}


impl FString {
    pub fn wrap(&self, width: usize) -> FString {
        let mut string = String::with_capacity(self.characters.len());

        for i in self.characters.iter() { string.push(to_output_placeholder(*i)) }

        // NOTE: This is not a fast implementation at all!!!!!
        let wrapped: Vec<char> = textwrap::fill(&string, textwrap::Options {
            break_words: true,
            width,
            initial_indent: "",
            subsequent_indent: "",
            wrap_algorithm: textwrap::WrapAlgorithm::new_optimal_fit(),
            word_separator: textwrap::WordSeparator::AsciiSpace,
            word_splitter: textwrap::WordSplitter::NoHyphenation,
        }).chars().collect();

        let mut orig_i = 0;
        let mut wrapped_i = 0;
        let mut results: Vec<FChar> = vec![];
        let mut current_formatting = Formatting::default();

        loop {
            match (self.characters.get(orig_i), wrapped.get(wrapped_i)) {
                (Some(orig), Some(new)) => {
                    update_formatting(&mut current_formatting, *orig);
                    if new.is_whitespace() {
                        if to_output_placeholder(*orig).is_whitespace() {
                            orig_i += 1;
                        } else {
                            results.push(from_output_placeholder(*new, current_formatting));
                            wrapped_i += 1;
                        }
                    } else if approximate_match(to_output_placeholder(*orig), *new) {
                        results.push(from_output_placeholder(*new, current_formatting));
                        orig_i += 1;
                        wrapped_i += 1;
                    } else {
                        panic!("wtf: {:?} {:?}", *orig, new)
                    }
                }
                (None, None) => { break; }
                (None, Some(new)) => {
                    results.push(from_output_placeholder(*new, current_formatting));
                    wrapped_i += 1;
                }
                (Some(old), None) => {
                    results.push(*old);
                    orig_i += 1;
                }
            }
        }

        FString { characters: results }
    }
}

fn approximate_match(orig: char, new: char) -> bool {
    if orig == new { return true }
    if orig.is_whitespace() && new.is_whitespace() { return true }
    return false
}