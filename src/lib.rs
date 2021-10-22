#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::non_ascii_literal)]

#[cfg(test)]
mod tests;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[allow(clippy::struct_excessive_bools)]
pub struct PinyinParser {
    p_strict: bool,
    p_preserve_punctuations: bool,
    p_preserve_spaces: bool,
    p_preserve_miscellaneous: bool,
}

impl Default for PinyinParser {
    fn default() -> Self {
        Self::new()
    }
}

impl PinyinParser {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            p_strict: false,
            p_preserve_spaces: false,
            p_preserve_punctuations: false,
            p_preserve_miscellaneous: false,
        }
    }

    #[must_use]
    pub const fn is_strict(self, b: bool) -> Self {
        Self {
            p_strict: b,
            ..self
        }
    }

    #[must_use]
    pub const fn preserve_spaces(self, b: bool) -> Self {
        Self {
            p_preserve_spaces: b,
            ..self
        }
    }

    #[must_use]
    pub const fn preserve_punctuations(self, b: bool) -> Self {
        Self {
            p_preserve_punctuations: b,
            ..self
        }
    }

    /// ```
    /// use pinyin_parser::PinyinParser;
    /// let parser = PinyinParser::new()
    ///     .is_strict(true)
    ///     .preserve_miscellaneous(true);
    /// assert_eq!(
    ///     parser
    ///         .parse("你Nǐ 好hǎo")
    ///         .into_iter()
    ///         .collect::<Vec<_>>(),
    ///     vec!["你", "nǐ", "好", "hǎo"]
    /// )
    /// ```
    #[must_use]
    pub const fn preserve_miscellaneous(self, b: bool) -> Self {
        Self {
            p_preserve_miscellaneous: b,
            ..self
        }
    }

    /// ```
    /// use pinyin_parser::PinyinParser;
    /// let parser = PinyinParser::new()
    ///     .is_strict(true)
    ///     .preserve_punctuations(true)
    ///     .preserve_spaces(true);
    /// assert_eq!(
    ///     parser
    ///         .parse("Nǐ zuò shénme?")
    ///         .into_iter()
    ///         .collect::<Vec<_>>(),
    ///     vec!["nǐ", " ", "zuò", " ", "shén", "me", "?"]
    /// )
    /// ```
    #[must_use]
    pub fn parse(self, s: &str) -> PinyinParserIter {
        PinyinParserIter {
            configs: self,
            it: VecAndIndex {
                vec: UnicodeSegmentation::graphemes(s, true)
                    .map(|c| pinyin_token::to_token(c, self.p_strict))
                    .collect::<Vec<_>>(),
                next_pos: 0,
            },
            state: ParserState::BeforeWordInitial,
        }
    }

    /// Strict mode:
    /// * forbids the use of breve instead of hacek to represent the third tone
    /// * forbids the use of IPA `ɡ` (U+0261) instead of `g`, and other such lookalike characters
    /// * allows apostrophes only before an `a`, an `e` or an `o`
    /// ```
    /// use pinyin_parser::PinyinParser;
    /// assert_eq!(
    ///     PinyinParser::strict("jīntiān")
    ///         .into_iter()
    ///         .collect::<Vec<_>>(),
    ///     vec!["jīn", "tiān"]
    /// );
    /// ```

    /// ```should_panic
    /// use pinyin_parser::PinyinParser;
    /// assert_eq!(
    ///     PinyinParser::strict("zǒnɡshì") // this `ɡ` is not the `g` from ASCII
    ///         .into_iter()
    ///         .collect::<Vec<_>>(),
    ///     vec!["zǒng", "shì"]
    /// );
    /// ```

    /// ```should_panic
    /// use pinyin_parser::PinyinParser;
    /// assert_eq!(
    ///     // An apostrophe can come only before an `a`, an `e` or an `o` in strict mode    
    ///     PinyinParser::strict("Yīng'guó")
    ///         .into_iter()
    ///         .collect::<Vec<_>>(),
    ///     vec!["yīng", "guó"]
    /// );
    /// ```

    /// This parser supports the use of `ẑ`, `ĉ`, `ŝ` and `ŋ`, though I have never seen anyone use it.
    /// ```
    /// use pinyin_parser::PinyinParser;
    /// assert_eq!(
    ///     PinyinParser::strict("Ẑāŋ").into_iter().collect::<Vec<_>>(),
    ///     vec!["zhāng"]
    /// )
    /// ```

    #[must_use]
    pub fn strict(s: &str) -> PinyinParserIter {
        Self::new().is_strict(true).parse(s)
    }

    /// ```
    /// use pinyin_parser::PinyinParser;
    /// assert_eq!(
    ///     // 'ă' is LATIN SMALL LETTER A WITH BREVE and is not accepted in strict mode.  
    ///     // The correct alphabet to use is 'ǎ'.  
    ///     PinyinParser::loose("mián'ăo")
    ///         .into_iter()
    ///         .collect::<Vec<_>>(),
    ///     vec!["mián", "ǎo"]
    /// );
    /// ```

    /// ```
    /// use pinyin_parser::PinyinParser;
    /// assert_eq!(
    ///     // An apostrophe can come only before an `a`, an `e` or an `o` in strict mode,
    ///     // but allowed here because it's loose    
    ///     PinyinParser::loose("Yīng'guó")
    ///         .into_iter()
    ///         .collect::<Vec<_>>(),
    ///     vec!["yīng", "guó"]
    /// );
    /// ```
    #[must_use]
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
            panic!("too much rewind");
        }
        self.next_pos -= n;
    }

    fn advance(&mut self, n: usize) {
        self.next_pos += n;
    }
}

impl Iterator for PinyinParserIter {
    type Item = String;

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)]
    fn next(&mut self) -> Option<Self::Item> {
        use pinyin_token::Alphabet;
        use pinyin_token::PinyinToken::{
            Alph, Apostrophe, LightToneMarker, Others, Punctuation, Space,
        };
        use ParserState::{
            AfterSyllablePossiblyConsumingApostrophe, BeforeWordInitial, InitialParsed, ZCSParsed,
        };
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
                (None, AfterSyllablePossiblyConsumingApostrophe | BeforeWordInitial) => {
                    return None
                }
                (None, InitialParsed(initial)) => {
                    panic!("unexpected end of string found after {:?}", initial);
                }
                (None, ZCSParsed(zcs)) => panic!("unexpected end of string found after {:?}", zcs),
                (
                    Some(Punctuation(s)),
                    BeforeWordInitial | AfterSyllablePossiblyConsumingApostrophe,
                ) => {
                    if self.configs.p_preserve_punctuations {
                        self.state = BeforeWordInitial;
                        return Some((*s).clone());
                    }
                    continue;
                }
                (Some(Space(s)), BeforeWordInitial | AfterSyllablePossiblyConsumingApostrophe) => {
                    if self.configs.p_preserve_spaces {
                        self.state = BeforeWordInitial;
                        return Some((*s).clone());
                    }
                    continue;
                }

                (Some(Others(s)), BeforeWordInitial | AfterSyllablePossiblyConsumingApostrophe) => {
                    if self.configs.p_preserve_miscellaneous {
                        self.state = BeforeWordInitial;
                        return Some((*s).clone());
                    }
                    continue;
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
                            return Some(alph.to_str(self.configs.p_strict));
                        }
                    }
                    Alphabet::F => self.state = InitialParsed(SpellingInitial::F),
                    Alphabet::D => self.state = InitialParsed(SpellingInitial::D),
                    Alphabet::T => self.state = InitialParsed(SpellingInitial::T),
                    Alphabet::N => {
                        if alph.diacritics.is_empty() {
                            self.state = InitialParsed(SpellingInitial::N);
                        } else {
                            return Some(alph.to_str(self.configs.p_strict));
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
                            self.state = ZCSParsed(ZCS::Z);
                        } else if matches!(
                            &alph.diacritics[..],
                            &[pinyin_token::Diacritic::Circumflex]
                        ) {
                            self.state = InitialParsed(SpellingInitial::ZH);
                        } else {
                            return Some(alph.to_str(self.configs.p_strict));
                        }
                    }
                    Alphabet::C => {
                        if alph.diacritics.is_empty() {
                            self.state = ZCSParsed(ZCS::C);
                        } else if matches!(
                            &alph.diacritics[..],
                            &[pinyin_token::Diacritic::Circumflex]
                        ) {
                            self.state = InitialParsed(SpellingInitial::CH);
                        } else {
                            return Some(alph.to_str(self.configs.p_strict));
                        }
                    }
                    Alphabet::S => {
                        if alph.diacritics.is_empty() {
                            self.state = ZCSParsed(ZCS::S);
                        } else if matches!(
                            &alph.diacritics[..],
                            &[pinyin_token::Diacritic::Circumflex]
                        ) {
                            self.state = InitialParsed(SpellingInitial::SH);
                        } else {
                            return Some(alph.to_str(self.configs.p_strict));
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

                (Some(Alph(alph)), ZCSParsed(zcs)) => {
                    if alph.alphabet == Alphabet::H {
                        self.state = match zcs {
                            ZCS::Z => InitialParsed(SpellingInitial::ZH),
                            ZCS::C => InitialParsed(SpellingInitial::CH),
                            ZCS::S => InitialParsed(SpellingInitial::SH),
                        }
                    } else {
                        self.it.rewind(1);
                        self.state = match zcs {
                            ZCS::Z => InitialParsed(SpellingInitial::Z),
                            ZCS::C => InitialParsed(SpellingInitial::C),
                            ZCS::S => InitialParsed(SpellingInitial::S),
                        }
                    }
                }

                (Some(Alph(_)), InitialParsed(initial)) => {
                    use finals::Candidate;
                    self.it.rewind(1);
                    let candidates = self.it.get_candidates_without_rhotic(self.configs.p_strict);

                    if candidates.is_empty() {
                        panic!(
                            "no adequate candidate for finals (-an, -ian, ...) is found, after the initial {:?}",
                            initial
                        );
                    }

                    for Candidate { ŋ, fin, tone } in candidates.clone() {
                        let fin_len = fin.len() - if ŋ { 1 } else { 0 }; // ŋ accounts for ng, hence the len is shorter by 1
                        self.it.advance(fin_len);

                        // ITERATOR IS TEMPORARILY ADVANCED HERE
                        match self.it.peek(0) {
                            None => {
                                self.it.advance(1);
                                self.state = AfterSyllablePossiblyConsumingApostrophe;
                                return Some(format!(
                                    "{}{}",
                                    initial,
                                    finals::FinalWithTone { fin, tone }
                                ));
                            }

                            Some(Apostrophe) => {
                                self.it.advance(1);

                                // In the strict mode, `a`, `e` or `o` must follow the apostrophe
                                if self.configs.p_strict {
                                    let a_e_o = match self.it.peek(0) {
                                        Some(Alph(a)) => matches!(
                                            a.alphabet,
                                            Alphabet::A | Alphabet::E | Alphabet::O
                                        ),
                                        _ => false,
                                    };

                                    if !a_e_o {
                                        panic!("In strict mode, an apostrophe must be followed by either 'a', 'e' or 'o'");
                                    }
                                }

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
                                    }
                                    // this is rhotic
                                    self.it.advance(1);
                                    self.state = AfterSyllablePossiblyConsumingApostrophe;
                                    return Some(format!(
                                        "{}{}r",
                                        initial,
                                        finals::FinalWithTone { fin, tone }
                                    ));
                                }

                                Alphabet::G =>
                                /* possibly g */
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
                                        // cannot be an additiona g
                                        // peeking `g` was not needed
                                        // hence simply return
                                        self.state = AfterSyllablePossiblyConsumingApostrophe;
                                        return Some(format!(
                                            "{}{}",
                                            initial,
                                            finals::FinalWithTone { fin, tone }
                                        ));
                                    }
                                    // this candidate is wrong
                                    self.it.rewind(fin_len);
                                    continue;
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
                                        // peeking `n` was not needed
                                        // hence simply return
                                        self.state = AfterSyllablePossiblyConsumingApostrophe;
                                        return Some(format!(
                                            "{}{}",
                                            initial,
                                            finals::FinalWithTone { fin, tone }
                                        ));
                                    }
                                    // this candidate is not good
                                    self.it.rewind(fin_len);
                                    continue;
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
                    panic!(
                        "no adequate candidate for finals (-an, -ian, ...) found, among possible candidates {:?}",
                        candidates
                    );
                }
            }
        }
    }
}

mod finals;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ZCS {
    Z,
    C,
    S,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum SpellingInitial {
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
