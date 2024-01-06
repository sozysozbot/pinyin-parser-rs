use crate::PinyinParser;
#[test]
fn test_strict1() {
    assert_eq!(
        PinyinParser::strict("mián'ǎo").collect::<Vec<_>>(),
        vec!["mián", "ǎo"]
    );
}

#[test]
fn test_strict3() {
    assert_eq!(
        PinyinParser::strict("Nǐ zuò shénme?").collect::<Vec<_>>(),
        vec!["nǐ", "zuò", "shén", "me"]
    );
}

#[test]
fn test_strict4() {
    assert_eq!(
        PinyinParser::strict("Nǐ xiǎng qù nǎli?").collect::<Vec<_>>(),
        vec!["nǐ", "xiǎng", "qù", "nǎ", "li"]
    );
}

#[test]
fn test_strict5() {
    assert_eq!(
        PinyinParser::strict("jiǔshíjiǔ").collect::<Vec<_>>(),
        vec!["jiǔ", "shí", "jiǔ"]
    );
}

#[test]
fn test_strict6() {
    assert_eq!(
        PinyinParser::strict("Wǒ rènshi Lǜ xiǎojiě.").collect::<Vec<_>>(),
        vec!["wǒ", "rèn", "shi", "lǜ", "xiǎo", "jiě"]
    );
}

#[test]
fn test() {
    let parser = PinyinParser::new()
        .with_strictness(crate::Strictness::Strict)
        .preserve_miscellaneous(true)
        .preserve_spaces(true);
    assert_eq!(
        parser.parse("你Nǐ 好hǎo").collect::<Vec<_>>(),
        vec!["你", "nǐ", " ", "好", "hǎo"]
    );
}

#[test]
fn test2() {
    let parser = PinyinParser::new()
        .with_strictness(crate::Strictness::Strict)
        .preserve_miscellaneous(true)
        .preserve_spaces(true);
    assert_eq!(
        parser.parse("你Nǐあ好hǎo").collect::<Vec<_>>(),
        vec!["你", "nǐ", "あ", "好", "hǎo"]
    );
}

#[test]
fn test3() {
    assert_eq!(
        PinyinParser::strict("yù'ér").collect::<Vec<_>>(),
        vec!["yù", "ér"]
    );
}

#[test]
fn test4() {
    assert_eq!(
        PinyinParser::strict("yīdiǎnr").collect::<Vec<_>>(),
        vec!["yī", "diǎnr"]
    );
}

/*
#[test]
fn test_loose2() {
    assert_eq!(
        PinyinParser::loose("ni3 hao3")
            .collect::<Vec<_>>(),
        vec!["nǐ", "hǎo"]
    );
}

#[test]
fn test_loose3() {
    assert_eq!(
        PinyinParser::loose("mi2ngtian1")
            .collect::<Vec<_>>(),
        vec!["míng", "tiān"]
    );
}

*/
