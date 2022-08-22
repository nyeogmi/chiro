use super::{fstring::FString, fchar::{FChar, Formatting}};

impl FString {
    pub fn wrap(&self, width: usize) -> FString {
        let mut string = String::with_capacity(self.characters.len());
        for i in self.characters.iter() { string.push(i.character); }

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
                (Some(FChar { character: orig, formatting }), Some(new)) => {
                    if *new == '\n' {
                        if orig.is_whitespace() {
                            orig_i += 1;
                        } else {
                            results.push(FChar { character: *new, formatting: current_formatting});
                            current_formatting = *formatting;
                            results.push(FChar { character: *orig, formatting: current_formatting});
                            wrapped_i += 1 ;
                        }
                    } else if approximate_match(*orig, *new) {
                        current_formatting = *formatting;
                        results.push(FChar { character: *new, formatting: current_formatting});
                        orig_i += 1;
                        wrapped_i += 1;
                    } else {
                        panic!("wtf")
                    }
                }
                (None, None) => { break; }
                (None, Some(new)) => {
                    results.push(FChar { character: *new, formatting: current_formatting })
                }
                (Some(old), None) => {
                    results.push(*old)
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