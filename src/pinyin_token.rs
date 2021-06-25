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
    pub fn to_str_fixing_breve(&self) -> &'static str {
        use Diacritic::*;
        match self {
            Breve => Hacek.to_str(),
            a => a.to_str(),
        }
    }

    pub fn to_str(&self) -> &'static str {
        use Diacritic::*;
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
    pub wrong: bool,
    pub alphabet: Alphabet,
    pub diacritics: Vec<Diacritic>,
}

impl AlphabetWithDiacritics {
    pub fn to_str(&self, preserve_capitalization: bool, strict: bool) -> String {
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
    NG,
}

impl Alphabet {
    pub fn to_cap(self) -> &'static str {
        use Alphabet::*;
        match self {
            A => "A",
            B => "B",
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            H => "H",
            I => "I",
            J => "J",
            K => "K",
            L => "L",
            M => "M",
            N => "N",
            O => "O",
            P => "P",
            Q => "Q",
            R => "R",
            S => "S",
            T => "T",
            U => "U",
            W => "W",
            X => "X",
            Y => "Y",
            Z => "Z",
            NG => "Ŋ",
        }
    }

    pub fn to_low(self) -> &'static str {
        use Alphabet::*;
        match self {
            A => "a",
            B => "b",
            C => "c",
            D => "d",
            E => "e",
            F => "f",
            G => "g",
            H => "h",
            I => "i",
            J => "j",
            K => "k",
            L => "l",
            M => "m",
            N => "n",
            O => "o",
            P => "p",
            Q => "q",
            R => "r",
            S => "s",
            T => "t",
            U => "u",
            W => "w",
            X => "x",
            Y => "y",
            Z => "z",
            NG => "ŋ",
        }
    }
}

macro_rules! low {
    ($u:ident) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: false,
            wrong: false,
            alphabet: $u,
            diacritics: vec![],
        })
    };

    ($u:ident, $($arg:tt)*) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: false,
            wrong: false,
            alphabet: $u,
            diacritics: vec![$($arg)*],
        })
    };
}

macro_rules! wrong_low {
    ($u:expr) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: false,
            wrong: true,
            alphabet: $u,
            diacritics: vec![],
        })
    };

    ($u:expr, $($arg:tt)*) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: false,
            wrong: true,
            alphabet: $u,
            diacritics: vec![$($arg)*],
        })
    };
}

macro_rules! cap {
    ($u:expr) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: true,
            wrong: false,
            alphabet: $u,
            diacritics: vec![],
        })
    };

    ($u:expr, $($arg:tt)*) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: true,
            wrong: false,
            alphabet: $u,
            diacritics: vec![$($arg)*],
        })
    };
}

macro_rules! wrong_cap {
    ($u:ident) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: true,
            wrong: true,
            alphabet: $u,
            diacritics: vec![],
        })
    };

    ($u:ident, $($arg:tt)*) => {
        PinyinToken::Alph(AlphabetWithDiacritics {
            capitalized: true,
            wrong: true,
            alphabet: $u,
            diacritics: vec![$($arg)*],
        })
    };
}

pub fn to_token(s: &str) -> PinyinToken {
    use Alphabet::*;
    use Diacritic::*;
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
        Some('x') => cap!(X), Some('X') => cap!(X),
        Some('y') => cap!(Y), Some('Y') => cap!(Y),
        Some('z') => cap!(Z), Some('Z') => cap!(Z),

        Some('ĉ') => low!(C, Circumflex), Some('Ĉ') => cap!(C, Circumflex),
        Some('ŝ') => low!(S, Circumflex), Some('Ŝ') => cap!(S, Circumflex),
        Some('ẑ') => low!(Z, Circumflex), Some('Ẑ') => cap!(Z, Circumflex),
        Some('ŋ') => low!(NG), Some('Ŋ') => cap!(NG),

        Some('v') => low!(U, Umlaut), Some('V') => cap!(U, Umlaut),
        Some('ü') => low!(U, Umlaut), Some('Ü') => cap!(U, Umlaut),
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
        Some('\u{0261}') /* IPA's /g/ */ => wrong_low!(G),
        Some('\u{0251}') /* IPA's /ɑ/ */ => wrong_low!(A),
        // greek capital letters
        Some('Α') => wrong_cap!(A), Some('Β') => wrong_cap!(B), Some('Ε') => wrong_cap!(E),
        Some('Ζ') => wrong_cap!(Z), Some('Η') => wrong_cap!(H), Some('Ι') => wrong_cap!(I),
        Some('Κ') => wrong_cap!(K), Some('Μ') => wrong_cap!(M), Some('Ν') => wrong_cap!(N),
        Some('Ο') => wrong_cap!(O), Some('Ρ') => wrong_cap!(P), Some('Τ') => wrong_cap!(T),
        Some('Υ') => wrong_cap!(Y), Some('Χ') => wrong_cap!(X),
        Some('ο') => wrong_low!(O), Some('α') => wrong_low!(A),

        // others
        Some('·') => PinyinToken::LightToneMarker,
        Some('\'' | '’') => PinyinToken::Apostrophe,
        Some(' ' | '!' | '-' | '?' | '—' | '…') => PinyinToken::Punctuation(s.to_owned()),
        Some(q) => if q.is_whitespace() {PinyinToken::Space(s.to_owned())} else {PinyinToken::Others(s.to_owned())}
    };

    match base {
        PinyinToken::Alph(alph) => {
            let mut alph = alph;
            for d in it {
                match d {
                    '\u{304}' => alph.diacritics.push(Macron),
                    '\u{301}' => alph.diacritics.push(Acute),
                    '\u{30c}' => alph.diacritics.push(Hacek),
                    '\u{300}' => alph.diacritics.push(Grave),
                    '\u{306}' => alph.diacritics.push(Breve),
                    '\u{308}' => alph.diacritics.push(Umlaut),
                    '\u{302}' => alph.diacritics.push(Circumflex),
                    _ => return PinyinToken::Others(s.to_owned()),
                }
            }
            PinyinToken::Alph(alph)
        }
        _ => base,
    }
}
