use crate::*;
#[test]
fn test_strict0() {
    assert_eq!(
        PinyinParser::strict("jīntiān")
            .into_iter()
            .collect::<Vec<_>>(),
        vec!["jīn", "tiān"]
    );
}

#[test]
fn test_strict1() {
    assert_eq!(
        PinyinParser::strict("mián'ǎo")
            .into_iter()
            .collect::<Vec<_>>(),
        vec!["mián", "ǎo"]
    );
}

#[test]
fn test_strict2() {
    // this is officially allowed, though I have never seen anyone use it
    assert_eq!(
        PinyinParser::strict("Ẑāŋ").into_iter().collect::<Vec<_>>(),
        vec!["zhāng"]
    )
}

#[test]
fn test_strict3() {
    assert_eq!(
        PinyinParser::strict("Nǐ zuò shénme?")
            .into_iter()
            .collect::<Vec<_>>(),
        vec!["nǐ", "zuò", "shén", "me"]
    )
}

#[test]
fn test_strict4() {
    assert_eq!(
        PinyinParser::strict("Nǐ xiǎng qù nǎli?")
            .into_iter()
            .collect::<Vec<_>>(),
        vec!["nǐ", "xiǎng", "qù", "nǎ", "li"]
    )
}

#[test]
fn test_strict5() {
    assert_eq!(
        PinyinParser::strict("jiǔshíjiǔ")
            .into_iter()
            .collect::<Vec<_>>(),
        vec!["jiǔ", "shí", "jiǔ"]
    )
}

#[test]
fn test_new() {
    let parser = PinyinParser::new()
        .is_strict(true)
        .preserve_punctuations(true)
        .preserve_spaces(true);
    assert_eq!(
        parser
            .parse("Nǐ zuò shénme?")
            .into_iter()
            .collect::<Vec<_>>(),
        vec!["nǐ", " ", "zuò", " ", "shén", "me", "?"]
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
/*
#[test]
fn test_loose2() {
    assert_eq!(
        PinyinParser::loose("ni3 hao3")
            .into_iter()
            .collect::<Vec<_>>(),
        vec!["nǐ", "hǎo"]
    );
}

#[test]
fn test_loose3() {
    assert_eq!(
        PinyinParser::loose("mi2ngtian1")
            .into_iter()
            .collect::<Vec<_>>(),
        vec!["míng", "tiān"]
    );
}
*/
