#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PinyinToken {
    Alph(AlphabetWithDiacritics),
    LightToneMarker,
    Punctuation(String),
    Apostrophe,
    Space(String),
    Others(String),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Diacritic {
    Umlaut,     /* ü */
    Macron,     /* fist tone */
    Acute,      /* second tone */
    Hacek,      /* third tone */
    Breve,      /* wrong third tone */
    Grave,      /* fourth tone */
    Circumflex, /* ĉ, ê */
}

impl Diacritic {
    #[must_use]
    pub const fn to_str_fixing_breve(&self) -> &'static str {
        use Diacritic::{Breve, Hacek};
        match self {
            Breve => Hacek.to_str(),
            a => a.to_str(),
        }
    }

    #[must_use]
    pub const fn to_str(&self) -> &'static str {
        use Diacritic::{Acute, Breve, Circumflex, Grave, Hacek, Macron, Umlaut};
        match self {
            Macron => "\u{304}",
            Acute => "\u{301}",
            Hacek => "\u{30c}",
            Grave => "\u{300}",
            Breve => "\u{306}",
            Umlaut => "\u{308}",
            Circumflex => "\u{302}",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlphabetWithDiacritics {
    pub capitalized: bool,
    pub alphabet: Alphabet,
    pub diacritics: Vec<Diacritic>,
}

impl AlphabetWithDiacritics {
    pub fn to_str(&self, strict: bool) -> String { self.to_str_preserving_capitalization(false, strict)}

    pub fn to_str_preserving_capitalization(&self, preserve_capitalization: bool, strict: bool) -> String {
        use unicode_normalization::UnicodeNormalization;
        let base = if preserve_capitalization && self.capitalized {
            self.alphabet.to_cap()
        } else {
            self.alphabet.to_low()
        };

        let diacritics = self
            .diacritics
            .iter()
            .map(|d| {
                if strict {
                    d.to_str()
                } else {
                    d.to_str_fixing_breve()
                }
            })
            .collect::<String>();

        format!("{}{}", base, diacritics).nfc().collect::<String>()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Alphabet {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    /* V is treated as ü */
    W,
    X,
    Y,
    Z,
    Ŋ,
}

impl Alphabet {
    #[must_use]
    pub const fn to_cap(self) -> &'static str {
        match self {
            Alphabet::A => "A",
            Alphabet::B => "B",
            Alphabet::C => "C",
            Alphabet::D => "D",
            Alphabet::E => "E",
            Alphabet::F => "F",
            Alphabet::G => "G",
            Alphabet::H => "H",
            Alphabet::I => "I",
            Alphabet::J => "J",
            Alphabet::K => "K",
            Alphabet::L => "L",
            Alphabet::M => "M",
            Alphabet::N => "N",
            Alphabet::O => "O",
            Alphabet::P => "P",
            Alphabet::Q => "Q",
            Alphabet::R => "R",
            Alphabet::S => "S",
            Alphabet::T => "T",
            Alphabet::U => "U",
            Alphabet::W => "W",
            Alphabet::X => "X",
            Alphabet::Y => "Y",
            Alphabet::Z => "Z",
            Alphabet::Ŋ => "Ŋ",
        }
    }

    #[must_use]
    pub const fn to_low(self) -> &'static str {
        match self {
            Alphabet::A => "a",
            Alphabet::B => "b",
            Alphabet::C => "c",
            Alphabet::D => "d",
            Alphabet::E => "e",
            Alphabet::F => "f",
            Alphabet::G => "g",
            Alphabet::H => "h",
            Alphabet::I => "i",
            Alphabet::J => "j",
            Alphabet::K => "k",
            Alphabet::L => "l",
            Alphabet::M => "m",
            Alphabet::N => "n",
            Alphabet::O => "o",
            Alphabet::P => "p",
            Alphabet::Q => "q",
            Alphabet::R => "r",
            Alphabet::S => "s",
            Alphabet::T => "t",
            Alphabet::U => "u",
            Alphabet::W => "w",
            Alphabet::X => "x",
            Alphabet::Y => "y",
            Alphabet::Z => "z",
            Alphabet::Ŋ => "ŋ",
        }
    }
}

macro_rules! low {
    ($u:ident) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: false,
            alphabet: $u,
            diacritics: vec![],
        })
    };

    ($u:ident, $($arg:tt)*) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: false,
            alphabet: $u,
            diacritics: vec![$($arg)*],
        })
    };
}

macro_rules! cap {
    ($u:expr) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: true,
            alphabet: $u,
            diacritics: vec![],
        })
    };

    ($u:expr, $($arg:tt)*) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: true,
            alphabet: $u,
            diacritics: vec![$($arg)*],
        })
    };
}

#[allow(clippy::too_many_lines)]
pub fn to_token(s: &str, strict: bool) -> PinyinToken {
    use Alphabet::{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, W, X, Y, Z, Ŋ};
    use Diacritic::{Acute, Breve, Circumflex, Grave, Hacek, Macron, Umlaut};
    let mut it = s.chars();
    let base = match it.next() {
        None => panic!("to_token received empty string"),
        Some('a') => low!(A), Some('A') => cap!(A),
        Some('b') => low!(B), Some('B') => cap!(B),
        Some('c') => low!(C), Some('C') => cap!(C),
        Some('d') => low!(D), Some('D') => cap!(D),
        Some('e') => low!(E), Some('E') => cap!(E),
        Some('f') => low!(F), Some('F') => cap!(F),
        Some('g') => low!(G), Some('G') => cap!(G),
        Some('h') => low!(H), Some('H') => cap!(H),
        Some('i') => low!(I), Some('I') => cap!(I),
        Some('j') => low!(J), Some('J') => cap!(J),
        Some('k') => low!(K), Some('K') => cap!(K),
        Some('l') => low!(L), Some('L') => cap!(L),
        Some('m') => low!(M), Some('M') => cap!(M),
        Some('n') => low!(N), Some('N') => cap!(N),
        Some('o') => low!(O), Some('O') => cap!(O),
        Some('p') => low!(P), Some('P') => cap!(P),
        Some('q') => low!(Q), Some('Q') => cap!(Q),
        Some('r') => low!(R), Some('R') => cap!(R),
        Some('s') => low!(S), Some('S') => cap!(S),
        Some('t') => low!(T), Some('T') => cap!(T),
        Some('u') => low!(U), Some('U') => cap!(U),
        Some('w') => low!(W), Some('W') => cap!(W),
        Some('x') => low!(X), Some('X') => cap!(X),
        Some('y') => low!(Y), Some('Y') => cap!(Y),
        Some('z') => low!(Z), Some('Z') => cap!(Z),

        Some('ĉ') => low!(C, Circumflex), Some('Ĉ') => cap!(C, Circumflex),
        Some('ŝ') => low!(S, Circumflex), Some('Ŝ') => cap!(S, Circumflex),
        Some('ẑ') => low!(Z, Circumflex), Some('Ẑ') => cap!(Z, Circumflex),
        Some('ŋ') => low!(Ŋ), Some('Ŋ') => cap!(Ŋ),

        Some('v' | 'ü') => low!(U, Umlaut), Some('V' | 'Ü') => cap!(U, Umlaut),
        Some('ê') => low!(E, Circumflex), Some('Ê') => cap!(E, Circumflex),

        // first tone -- macron
        Some('ā') => low!(A, Macron), Some('Ā') => cap!(A, Macron),
        Some('ē') => low!(E, Macron), Some('Ē') => cap!(E, Macron),
        Some('ī') => low!(I, Macron), Some('Ī') => cap!(I, Macron),
        Some('ō') => low!(O, Macron), Some('Ō') => cap!(O, Macron),
        Some('ū') => low!(U, Macron), Some('Ū') => cap!(U, Macron),
        Some('ǖ') => low!(U, Umlaut, Macron), Some('Ǖ') => cap!(U, Umlaut, Macron),

        // second tone -- acute
        Some('á') => low!(A, Acute), Some('Á') => cap!(A, Acute),
        Some('é') => low!(E, Acute), Some('É') => cap!(E, Acute),
        Some('í') => low!(I, Acute), Some('Í') => cap!(I, Acute),
        Some('ó') => low!(O, Acute), Some('Ó') => cap!(O, Acute),
        Some('ú') => low!(U, Acute), Some('Ú') => cap!(U, Acute),
        Some('ǘ') => low!(U, Umlaut, Acute), Some('Ǘ') => cap!(U, Umlaut, Acute),
        Some('ế') => low!(E, Circumflex, Acute), Some('Ế') => cap!(E, Circumflex, Acute),
        Some('ḿ') => low!(M, Acute), Some('Ḿ') => cap!(M, Acute),
        Some('ń') => low!(N, Acute), Some('Ń') => cap!(N, Acute),

        // third tone -- hacek
        Some('ǎ') => low!(A, Hacek), Some('Ǎ') => cap!(A, Hacek),
        Some('ě') => low!(E, Hacek), Some('Ě') => cap!(E, Hacek),
        Some('ǐ') => low!(I, Hacek), Some('Ǐ') => cap!(I, Hacek),
        Some('ǒ') => low!(O, Hacek), Some('Ǒ') => cap!(O, Hacek),
        Some('ǔ') => low!(U, Hacek), Some('Ǔ') => cap!(U, Hacek),
        Some('ǚ') => low!(U, Umlaut, Hacek), Some('Ǚ') => cap!(U, Umlaut, Hacek),
        Some('ň') => low!(N, Hacek), Some('Ň') => cap!(N, Hacek),

        // wrong third tone -- breve
        Some('ă') => low!(A, Breve), Some('Ă') => cap!(A, Breve),
        Some('ĕ') => low!(E, Breve), Some('Ĕ') => cap!(E, Breve),
        Some('ĭ') => low!(I, Breve), Some('Ĭ') => cap!(I, Breve),
        Some('ŏ') => low!(O, Breve), Some('Ŏ') => cap!(O, Breve),
        Some('ŭ') => low!(U, Breve), Some('Ŭ') => cap!(U, Breve),

        // fourth tone -- grave
        Some('à') => low!(A, Grave), Some('À') => cap!(A, Grave),
        Some('è') => low!(E, Grave), Some('È') => cap!(E, Grave),
        Some('ì') => low!(I, Grave), Some('Ì') => cap!(I, Grave),
        Some('ò') => low!(O, Grave), Some('Ò') => cap!(O, Grave),
        Some('ù') => low!(U, Grave), Some('Ù') => cap!(U, Grave),
        Some('ǜ') => low!(U, Umlaut, Grave), Some('Ǜ') => cap!(U, Umlaut, Grave),
        Some('ề') => low!(E, Circumflex, Grave), Some('Ề') => cap!(E, Circumflex, Grave),
        Some('ǹ') => low!(N, Grave), Some('Ǹ') => cap!(N, Grave),

        // wrong
        Some('\u{0261}') /* IPA's /g/ */ => if strict { panic!("'\u{0261}' looks like 'g', but it is not.") } else { low!(G) },
        Some(a @ ('\u{0251}' /* IPA's /ɑ/ */ | 'α')) => if strict { panic!("'{}' looks like 'a', but it is not.", a) } else { low!(A) },
        Some('ο') => if strict { panic!("'ο' looks like 'o', but it is not.") } else { low!(O) },
        // greek capital letters
        Some('Α') => if strict { panic!("'Α' looks like 'A', but it is not.") } else {cap!(A)}, 
        Some('Β') => if strict { panic!("'Β' looks like 'B', but it is not.") } else {cap!(B)}, 
        Some('Ε') => if strict { panic!("'Ε' looks like 'E', but it is not.") } else {cap!(E)},
        Some('Ζ') => if strict { panic!("'Ζ' looks like 'Z', but it is not.") } else {cap!(Z)}, 
        Some('Η') => if strict { panic!("'Η' looks like 'H', but it is not.") } else {cap!(H)}, 
        Some('Ι') => if strict { panic!("'Ι' looks like 'I', but it is not.") } else {cap!(I)},
        Some('Κ') => if strict { panic!("'Κ' looks like 'K', but it is not.") } else {cap!(K)}, 
        Some('Μ') => if strict { panic!("'Μ' looks like 'M', but it is not.") } else {cap!(M)} , 
        Some('Ν') => if strict { panic!("'Ν' looks like 'N', but it is not.") } else {cap!(N)},
        Some('Ο') => if strict { panic!("'Ο' looks like 'O', but it is not.") } else {cap!(O)}, 
        Some('Ρ') => if strict { panic!("'Ρ' looks like 'P', but it is not.") } else {cap!(P)}, 
        Some('Τ') => if strict { panic!("'Τ' looks like 'T', but it is not.") } else {cap!(T)},
        Some('Υ') => if strict { panic!("'Υ' looks like 'Y', but it is not.") } else {cap!(Y)}, 
        Some('Χ') => if strict { panic!("'Χ' looks like 'X', but it is not.") } else {cap!(X)},
        
        // others
        Some('·') => PinyinToken::LightToneMarker,
        Some('\'' | '’') => PinyinToken::Apostrophe,
        Some('!' | '-' | '?' | '—' | '…') => PinyinToken::Punctuation(s.to_owned()),
        Some(q) => if q.is_whitespace() {PinyinToken::Space(s.to_owned())} else {PinyinToken::Others(s.to_owned())}
    };

    match base {
        PinyinToken::Alph(alph) => {
            let mut alph = alph;
            for d in it {
                match diacritic(d) {
                    Some(a) => alph.diacritics.push(a),
                    _ => return PinyinToken::Others(s.to_owned()),
                }
            }
            PinyinToken::Alph(alph)
        }
        _ => base,
    }
}

const fn diacritic(c: char) -> Option<Diacritic> {
    match c {
        '\u{304}' => Some(Diacritic::Macron),
        '\u{301}' => Some(Diacritic::Acute),
        '\u{30c}' => Some(Diacritic::Hacek),
        '\u{300}' => Some(Diacritic::Grave),
        '\u{306}' => Some(Diacritic::Breve),
        '\u{308}' => Some(Diacritic::Umlaut),
        '\u{302}' => Some(Diacritic::Circumflex),
        _ => None
    }
}