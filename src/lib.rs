#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_strict1() {
        assert_eq!(
            PinyinParser::strict("mián'ǎo")
                .into_iter()
                .collect::<Vec<_>>(),
            vec!["mián", "ǎo"]
        );
    }

    fn test_strict2() {
        // this is officially allowed, though I have never seen anyone use it
        assert_eq!(
            PinyinParser::strict("Ẑāŋ").into_iter().collect::<Vec<_>>(),
            vec!["zhāng"]
        )
    }

    fn test_new() {
        let parser = PinyinParser::new()
            .is_strict(true)
            .preserve_punctuations(true)
            .preserve_spaces(true)
            .preserve_capitalization(true);
        assert_eq!(
            parser
                .parse("Nǐ zuò shénme?")
                .into_iter()
                .collect::<Vec<_>>(),
            vec!["Nǐ", " ", "zuò", " ", "shén", "me", "?"]
        )
    }

    #[test]
    fn test_loose1() {
        assert_eq!(
            PinyinParser::loose("mián'ăo") // ă is LATIN SMALL LETTER A WITH BREVE and is not accepted in strict mode.
                .into_iter()
                .collect::<Vec<_>>(),
            vec!["mián", "ǎo"]
        );
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct PinyinParser {
    _strict: bool,
    _allow_ambiguous: bool,
    _preserve_punctuations: bool,
    _preserve_spaces: bool,
    _preserve_capitalization: bool,
}

impl PinyinParser {
    pub fn new() -> Self {
        PinyinParser {
            _strict: false,
            _allow_ambiguous: false,
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
        todo!()
    }

    pub fn strict(s: &str) -> PinyinParserIter {
        Self::new().is_strict(true).parse(s)
    }

    pub fn loose(s: &str) -> PinyinParserIter {
        Self::new().parse(s)
    }
}

pub struct PinyinParserIter {}

impl Iterator for PinyinParserIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NonZeroInitial {
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
}

type Initial = Option<NonZeroInitial>;
