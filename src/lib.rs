#[cfg(test)]
mod tests;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[allow(clippy::struct_excessive_bools)]
pub struct PinyinParser {
    _strict: bool,
    _allow_ambiguous_elision_of_apostrophe: bool,
    _preserve_punctuations: bool,
    _preserve_spaces: bool,
    _preserve_capitalization: bool,
}

impl PinyinParser {
    pub fn new() -> Self {
        PinyinParser {
            _strict: false,
            _allow_ambiguous_elision_of_apostrophe: false,
            _preserve_spaces: false,
            _preserve_capitalization: false,
            _preserve_punctuations: false,
        }
    }

    pub fn is_strict(self, b: bool) -> Self {
        Self { _strict: b, ..self }
    }

    pub fn preserve_spaces(self, b: bool) -> Self {
        Self {
            _preserve_spaces: b,
            ..self
        }
    }

    pub fn preserve_capitalization(self, b: bool) -> Self {
        Self {
            _preserve_capitalization: b,
            ..self
        }
    }

    /// allow british spelling
    pub fn preserve_capitalisation(self, b: bool) -> Self {
        self.preserve_capitalization(b)
    }

    pub fn preserve_punctuations(self, b: bool) -> Self {
        Self {
            _preserve_spaces: b,
            ..self
        }
    }

    pub fn parse(self, s: &str) -> PinyinParserIter {
        PinyinParserIter {
            configs: self,
            remaining: UnicodeSegmentation::graphemes(s, true)
                .map(|c| pinyin_token::to_token(c))
                .collect::<Vec<_>>()
                .into_iter(),
            state: ParserState::BeforeWordInitial,
            buffer: vec![],
        }
    }

    pub fn strict(s: &str) -> PinyinParserIter {
        Self::new().is_strict(true).parse(s)
    }

    pub fn loose(s: &str) -> PinyinParserIter {
        Self::new().parse(s)
    }
}

mod pinyin_token;

pub struct PinyinParserIter {
    configs: PinyinParser,
    remaining: std::vec::IntoIter<pinyin_token::PinyinToken>,
    state: ParserState,
    buffer: Vec<pinyin_token::PinyinToken>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ParserState {
    BeforeWordInitial,
    InitialParsed(SpellingInitial),
    ZCSParsed(ZCS),
}

impl Iterator for PinyinParserIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        use pinyin_token::Alphabet;
        use pinyin_token::PinyinToken::*;
        use ParserState::*;
        loop {
            match (self.remaining.next(), self.state, &self.buffer[..]) {
                (None, BeforeWordInitial, &[]) => return None,
                (Some(Punctuation(s)), BeforeWordInitial, &[]) => {
                    if self.configs._preserve_punctuations {
                        return Some(s);
                    } else {
                        continue;
                    }
                }
                (Some(Space(s)), BeforeWordInitial, &[]) => {
                    if self.configs._preserve_spaces {
                        return Some(s);
                    } else {
                        continue;
                    }
                }

                (Some(Alph(alph)), BeforeWordInitial, &[]) => match alph.alphabet {
                    Alphabet::B => self.state = InitialParsed(SpellingInitial::B),
                    Alphabet::P => self.state = InitialParsed(SpellingInitial::P),
                    Alphabet::M => {
                        if alph.diacritics.is_empty() {
                            self.state = InitialParsed(SpellingInitial::M);
                        } else {
                            return Some(alph.to_str(
                                self.configs._preserve_capitalization,
                                self.configs._strict,
                            ));
                        }
                    }
                    Alphabet::F => self.state = InitialParsed(SpellingInitial::F),
                    Alphabet::D => self.state = InitialParsed(SpellingInitial::D),
                    Alphabet::T => self.state = InitialParsed(SpellingInitial::T),
                    Alphabet::N => {
                        if alph.diacritics.is_empty() {
                            self.state = InitialParsed(SpellingInitial::N)
                        } else {
                            return Some(alph.to_str(
                                self.configs._preserve_capitalization,
                                self.configs._strict,
                            ));
                        }
                    }
                    Alphabet::L => self.state = InitialParsed(SpellingInitial::L),
                    Alphabet::G => self.state = InitialParsed(SpellingInitial::G),
                    Alphabet::K => self.state = InitialParsed(SpellingInitial::K),
                    Alphabet::H => self.state = InitialParsed(SpellingInitial::H),
                    Alphabet::J => self.state = InitialParsed(SpellingInitial::J),
                    Alphabet::Q => self.state = InitialParsed(SpellingInitial::Q),
                    Alphabet::X => self.state = InitialParsed(SpellingInitial::X),
                    Alphabet::R => self.state = InitialParsed(SpellingInitial::R),
                    Alphabet::Y => self.state = InitialParsed(SpellingInitial::Y),
                    Alphabet::W => self.state = InitialParsed(SpellingInitial::W),
                    Alphabet::Z => {
                        if alph.diacritics.is_empty() {
                            self.state = ZCSParsed(ZCS::Z)
                        } else if matches!(
                            &alph.diacritics[..],
                            &[pinyin_token::Diacritic::Circumflex]
                        ) {
                            self.state = InitialParsed(SpellingInitial::ZH)
                        } else {
                            return Some(alph.to_str(
                                self.configs._preserve_capitalization,
                                self.configs._strict,
                            ));
                        }
                    }
                    Alphabet::C => {
                        if alph.diacritics.is_empty() {
                            self.state = ZCSParsed(ZCS::C)
                        } else if matches!(
                            &alph.diacritics[..],
                            &[pinyin_token::Diacritic::Circumflex]
                        ) {
                            self.state = InitialParsed(SpellingInitial::CH)
                        } else {
                            return Some(alph.to_str(
                                self.configs._preserve_capitalization,
                                self.configs._strict,
                            ));
                        }
                    }
                    Alphabet::S => {
                        if alph.diacritics.is_empty() {
                            self.state = ZCSParsed(ZCS::S)
                        } else if matches!(
                            &alph.diacritics[..],
                            &[pinyin_token::Diacritic::Circumflex]
                        ) {
                            self.state = InitialParsed(SpellingInitial::SH)
                        } else {
                            return Some(alph.to_str(
                                self.configs._preserve_capitalization,
                                self.configs._strict,
                            ));
                        }
                    }
                    Alphabet::A | Alphabet::E | Alphabet::O => {
                        self.buffer.push(Alph(alph));
                        self.state = InitialParsed(SpellingInitial::ZeroAEO);
                    }

                    Alphabet::I => todo!(),
                    Alphabet::U => todo!(),
                    Alphabet::NG => todo!(),
                },
                _ => todo!(),
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ZCS {
    Z,
    C,
    S,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SpellingInitial {
    B,
    P,
    M,
    F,
    D,
    T,
    N,
    L,
    G,
    K,
    H,
    J,
    Q,
    X,
    ZH,
    CH,
    SH,
    R,
    Z,
    C,
    S,
    Y,
    W,
    ZeroAEO,
}
