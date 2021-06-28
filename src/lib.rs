#[cfg(test)]
mod tests;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[allow(clippy::struct_excessive_bools)]
pub struct PinyinParser {
    _strict: bool,
    _preserve_punctuations: bool,
    _preserve_spaces: bool,
    _preserve_miscellaneous: bool,
}

impl Default for PinyinParser {
    fn default() -> Self {
        Self::new()
    }
}

impl PinyinParser {
    pub fn new() -> Self {
        PinyinParser {
            _strict: false,
            _preserve_spaces: false,
            _preserve_punctuations: false,
            _preserve_miscellaneous: false,
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

    pub fn preserve_punctuations(self, b: bool) -> Self {
        Self {
            _preserve_punctuations: b,
            ..self
        }
    }

    pub fn preserve_miscellaneous(self, b: bool) -> Self {
        Self {
            _preserve_miscellaneous: b,
            ..self
        }
    }

    pub fn parse(self, s: &str) -> PinyinParserIter {
        PinyinParserIter {
            configs: self,
            it: VecAndIndex {
                vec: UnicodeSegmentation::graphemes(s, true)
                    .map(|c| pinyin_token::to_token(c))
                    .collect::<Vec<_>>(),
                next_pos: 0,
            },
            state: ParserState::BeforeWordInitial,
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

struct VecAndIndex<T> {
    vec: std::vec::Vec<T>,
    next_pos: usize,
}

pub struct PinyinParserIter {
    configs: PinyinParser,
    it: VecAndIndex<pinyin_token::PinyinToken>,
    state: ParserState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ParserState {
    BeforeWordInitial,
    InitialParsed(SpellingInitial),
    ZCSParsed(ZCS),
    AfterSyllablePossiblyConsumingApostrophe,
}

impl<T> VecAndIndex<T> {
    fn next(&mut self) -> Option<&T> {
        let ans = self.vec.get(self.next_pos);
        self.next_pos += 1;
        ans
    }

    fn peek(&self, n: usize) -> Option<&T> {
        self.vec.get(self.next_pos + n)
    }

    fn rewind(&mut self, n: usize) {
        if self.next_pos < n {
            panic!("too much rewind")
        }
        self.next_pos -= n;
    }

    fn advance(&mut self, n: usize) {
        self.next_pos += n;
    }
}

impl Iterator for PinyinParserIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        use pinyin_token::Alphabet;
        use pinyin_token::PinyinToken::*;
        use ParserState::*;
        loop {
            match (self.it.next(), self.state) {
                (
                    b @ Some(LightToneMarker | Punctuation(_) | Apostrophe | Space(_) | Others(_)),
                    a @ (InitialParsed(_) | ZCSParsed(_)),
                ) => panic!("unexpected {:?} found after parsing initial {:?}", b, a),
                (
                    Some(LightToneMarker),
                    AfterSyllablePossiblyConsumingApostrophe | BeforeWordInitial,
                ) => continue, // just ignore it

                (
                    Some(Apostrophe),
                    AfterSyllablePossiblyConsumingApostrophe | BeforeWordInitial,
                ) => panic!("unexpected apostrophe found at the beginning of a word"),
                (None, AfterSyllablePossiblyConsumingApostrophe) => return None,
                (None, BeforeWordInitial) => return None,
                (None, InitialParsed(initial)) => {
                    panic!("unexpected end of string found after {:?}", initial)
                }
                (None, ZCSParsed(zcs)) => panic!("unexpected end of string found after {:?}", zcs),
                (
                    Some(Punctuation(s)),
                    BeforeWordInitial | AfterSyllablePossiblyConsumingApostrophe,
                ) => {
                    if self.configs._preserve_punctuations {
                        self.state = BeforeWordInitial;
                        return Some((*s).to_owned());
                    } else {
                        continue;
                    }
                }
                (Some(Space(s)), BeforeWordInitial | AfterSyllablePossiblyConsumingApostrophe) => {
                    if self.configs._preserve_spaces {
                        self.state = BeforeWordInitial;
                        return Some((*s).to_owned());
                    } else {
                        continue;
                    }
                }

                (Some(Others(s)), BeforeWordInitial | AfterSyllablePossiblyConsumingApostrophe) => {
                    if self.configs._preserve_miscellaneous {
                        self.state = BeforeWordInitial;
                        return Some((*s).to_owned());
                    } else {
                        continue;
                    }
                }

                (
                    Some(Alph(alph)),
                    BeforeWordInitial | AfterSyllablePossiblyConsumingApostrophe,
                ) => match alph.alphabet {
                    Alphabet::B => self.state = InitialParsed(SpellingInitial::B),
                    Alphabet::P => self.state = InitialParsed(SpellingInitial::P),
                    Alphabet::M => {
                        if alph.diacritics.is_empty() {
                            self.state = InitialParsed(SpellingInitial::M);
                        } else {
                            return Some(alph.to_str(self.configs._strict));
                        }
                    }
                    Alphabet::F => self.state = InitialParsed(SpellingInitial::F),
                    Alphabet::D => self.state = InitialParsed(SpellingInitial::D),
                    Alphabet::T => self.state = InitialParsed(SpellingInitial::T),
                    Alphabet::N => {
                        if alph.diacritics.is_empty() {
                            self.state = InitialParsed(SpellingInitial::N)
                        } else {
                            return Some(alph.to_str(self.configs._strict));
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
                            return Some(alph.to_str(self.configs._strict));
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
                            return Some(alph.to_str(self.configs._strict));
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
                            return Some(alph.to_str(self.configs._strict));
                        }
                    }
                    Alphabet::A | Alphabet::E | Alphabet::O => {
                        self.it.rewind(1);
                        self.state = InitialParsed(SpellingInitial::ZeroAEO);
                    }

                    Alphabet::I | Alphabet::U | Alphabet::Ŋ => panic!(
                        "unexpected alphabet {:?} found at the beginning of a word",
                        alph.alphabet,
                    ),
                },

                (Some(Alph(alph)), ZCSParsed(zcs)) => match alph.alphabet {
                    Alphabet::H => {
                        self.state = match zcs {
                            ZCS::Z => InitialParsed(SpellingInitial::ZH),
                            ZCS::C => InitialParsed(SpellingInitial::CH),
                            ZCS::S => InitialParsed(SpellingInitial::SH),
                        }
                    }
                    _ => {
                        self.it.rewind(1);
                        self.state = match zcs {
                            ZCS::Z => InitialParsed(SpellingInitial::Z),
                            ZCS::C => InitialParsed(SpellingInitial::C),
                            ZCS::S => InitialParsed(SpellingInitial::S),
                        }
                    }
                },

                (Some(Alph(_)), InitialParsed(initial)) => {
                    use finals::*;
                    self.it.rewind(1);
                    let candidates = self.it.get_candidates_without_rhotic(self.configs._strict);

                    for Candidate { ŋ, fin, tone } in candidates {
                        let fin_len = fin.len() - if ŋ { 1 } else { 0 }; // ŋ accounts for ng, hence the len is shorter by 1

                        println!(
                            "candidate: {:?}\nfin_len: {}\nremaining: {:?}\n\n",
                            Candidate { ŋ, fin, tone },
                            fin_len,
                            &self.it.vec[self.it.next_pos..]
                        );

                        self.it.advance(fin_len);

                        // ITERATOR IS TEMPORARILY ADVANCED HERE
                        match self.it.peek(0) {
                            None | Some(Apostrophe) => {
                                self.it.advance(1);
                                self.state = AfterSyllablePossiblyConsumingApostrophe;
                                return Some(format!(
                                    "{}{}",
                                    initial,
                                    finals::FinalWithTone { fin, tone }
                                ));
                            }

                            Some(Punctuation(_) | LightToneMarker | Space(_) | Others(_)) => {
                                self.state = AfterSyllablePossiblyConsumingApostrophe;
                                return Some(format!(
                                    "{}{}",
                                    initial,
                                    finals::FinalWithTone { fin, tone }
                                ));
                            }

                            Some(Alph(alph)) => match alph.alphabet {
                                Alphabet::A
                                | Alphabet::E
                                | Alphabet::I
                                | Alphabet::O
                                | Alphabet::U
                                | Alphabet::Ŋ => {
                                    /* we have read too much or too little; this candidate is not good; ignore. */
                                    self.it.rewind(fin_len);
                                    continue;
                                }

                                Alphabet::R =>
                                /* possibly rhotic */
                                {
                                    let vowel_follows = match self.it.peek(1) {
                                        Some(Alph(a)) => matches!(
                                            a.alphabet,
                                            Alphabet::A
                                                | Alphabet::E
                                                | Alphabet::I
                                                | Alphabet::O
                                                | Alphabet::U
                                        ),
                                        _ => false,
                                    };
                                    if vowel_follows {
                                        // cannot be rhotic
                                        // peeking `r` was not needed
                                        // hence simply return
                                        self.state = AfterSyllablePossiblyConsumingApostrophe;
                                        return Some(format!(
                                            "{}{}",
                                            initial,
                                            finals::FinalWithTone { fin, tone }
                                        ));
                                    } else {
                                        // this is rhotic
                                        self.it.advance(1);
                                        self.state = AfterSyllablePossiblyConsumingApostrophe;
                                        return Some(format!(
                                            "{}{}r",
                                            initial,
                                            finals::FinalWithTone { fin, tone }
                                        ));
                                    }
                                }

                                Alphabet::N => {
                                    let vowel_follows = match self.it.peek(1) {
                                        Some(Alph(a)) => matches!(
                                            a.alphabet,
                                            Alphabet::A
                                                | Alphabet::E
                                                | Alphabet::I
                                                | Alphabet::O
                                                | Alphabet::U
                                        ),
                                        _ => false,
                                    };
                                    if vowel_follows {
                                        // cannot be rhotic
                                        // peeking `r` was not needed
                                        // hence simply return
                                        self.state = AfterSyllablePossiblyConsumingApostrophe;
                                        return Some(format!(
                                            "{}{}",
                                            initial,
                                            finals::FinalWithTone { fin, tone }
                                        ));
                                    } else {
                                        // this candidate is not good
                                        self.it.rewind(fin_len);
                                        continue;
                                    }
                                }

                                _ => {
                                    self.state = AfterSyllablePossiblyConsumingApostrophe;
                                    return Some(format!(
                                        "{}{}",
                                        initial,
                                        finals::FinalWithTone { fin, tone }
                                    ));
                                }
                            },
                        }
                    }
                    panic!("no candidate found")
                }
            }
        }
    }
}

mod finals;

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

impl std::fmt::Display for SpellingInitial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            /* FIXME: capitalization is not preserved */
            SpellingInitial::B => write!(f, "b"),
            SpellingInitial::P => write!(f, "p"),
            SpellingInitial::M => write!(f, "m"),
            SpellingInitial::F => write!(f, "f"),
            SpellingInitial::D => write!(f, "d"),
            SpellingInitial::T => write!(f, "t"),
            SpellingInitial::N => write!(f, "n"),
            SpellingInitial::L => write!(f, "l"),
            SpellingInitial::G => write!(f, "g"),
            SpellingInitial::K => write!(f, "k"),
            SpellingInitial::H => write!(f, "h"),
            SpellingInitial::J => write!(f, "j"),
            SpellingInitial::Q => write!(f, "q"),
            SpellingInitial::X => write!(f, "x"),
            SpellingInitial::ZH => write!(f, "zh"),
            SpellingInitial::CH => write!(f, "ch"),
            SpellingInitial::SH => write!(f, "sh"),
            SpellingInitial::R => write!(f, "r"),
            SpellingInitial::Z => write!(f, "z"),
            SpellingInitial::C => write!(f, "c"),
            SpellingInitial::S => write!(f, "s"),
            SpellingInitial::Y => write!(f, "y"),
            SpellingInitial::W => write!(f, "w"),
            SpellingInitial::ZeroAEO => write!(f, ""),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct PinyinAmbiguousParser {
    _preserve_punctuations: bool,
    _preserve_spaces: bool,
    _preserve_capitalization: bool,
}

impl Default for PinyinAmbiguousParser {
    fn default() -> Self {
        Self::new()
    }
}

impl PinyinAmbiguousParser {
    pub fn new() -> Self {
        PinyinAmbiguousParser {
            _preserve_spaces: false,
            _preserve_capitalization: false,
            _preserve_punctuations: false,
        }
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

    pub fn parse(self, s: &str) -> PinyinAmbiguousParserIter {
        PinyinAmbiguousParserIter {
            configs: self,
            it: VecAndIndex {
                vec: UnicodeSegmentation::graphemes(s, true)
                    .map(|c| pinyin_token::to_token(c))
                    .collect::<Vec<_>>(),
                next_pos: 0,
            },
            state: ParserState::BeforeWordInitial,
        }
    }
}

pub struct PinyinAmbiguousParserIter {
    configs: PinyinAmbiguousParser,
    it: VecAndIndex<pinyin_token::PinyinToken>,
    state: ParserState,
}
