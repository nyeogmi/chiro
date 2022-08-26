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
        let new: Vec<char> = textwrap::fill(&string, textwrap::Options {
            break_words: true,
            width,
            initial_indent: "",
            subsequent_indent: "",
            wrap_algorithm: textwrap::WrapAlgorithm::new_optimal_fit(),
            word_separator: textwrap::WordSeparator::AsciiSpace,
            word_splitter: textwrap::WordSplitter::NoHyphenation,
        }).chars().collect();

        let mut old_i = 0;
        let mut new_i = 0;
        let mut results: Vec<FChar> = vec![];
        let mut current_formatting = Formatting::default();

        loop {
            match (self.characters.get(old_i), new.get(new_i)) {
                (Some(old), Some(new)) => {
                    update_formatting(&mut current_formatting, *old);

                    let new_ws = new.is_whitespace();
                    let old_ws = to_output_placeholder(*old).is_whitespace();

                    if new_ws && old_ws {
                        results.push(from_output_placeholder(*new, current_formatting));
                        old_i += 1;
                        new_i += 1;
                    } else if new_ws {
                        results.push(from_output_placeholder(*new, current_formatting));
                        new_i += 1;
                    } else if old_ws {
                        old_i += 1;
                    } else {
                        results.push(from_output_placeholder(*new, current_formatting));
                        old_i += 1;
                        new_i += 1;
                    }
                }
                (None, Some(new)) => {
                    results.push(from_output_placeholder(*new, current_formatting));
                    new_i += 1;
                }
                (Some(old), None) => {
                    update_formatting(&mut current_formatting, *old);
                    results.push(*old);
                    old_i += 1;
                }
                (None, None) => { break; }
            }
        }

        FString { characters: results }
    }
}