# pinyin-parser-rs

Parses a string of pinyin syllables. Covers marginal cases such as `ẑ`, `ŋ` and `ê`.

Since pinyin strings in the wild does not necessarily conform to the standard, this parser offers two modes: strict and loose.

Strict mode: 
* forbids the use of breve instead of hacek to represent the third tone
* forbids the use of IPA `ɡ` (U+0261) instead of `g`, and other such lookalike characters
* allows apostrophes only before an `a`, an `e` or an `o` 

## Examples

```rust
use pinyin_parser::PinyinParser;
assert_eq!(
    PinyinParser::strict("jīntiān")
        .into_iter()
        .collect::<Vec<_>>(),
    vec!["jīn", "tiān"]
);
```

Erhua is supported: (However, see [#2](https://github.com/sozysozbot/pinyin-parser-rs/issues/2))

```rust
use pinyin_parser::PinyinParser;
assert_eq!(
      PinyinParser::strict("yīdiǎnr")
          .collect::<Vec<_>>(),
      vec!["yī", "diǎnr"]
);
```

If you want `r` to be separated from the main syllable, use `.split_erhua()`

```rust
use pinyin_parser::PinyinParser;
assert_eq!(
    PinyinParser::strict("yīdiǎnr")
        .split_erhua()
        .collect::<Vec<_>>(),
    vec!["yī", "diǎn", "r"]
);
```

This parser supports the use of `ẑ`, `ĉ`, `ŝ` and `ŋ`, though I have never seen anyone use it.
```rust
use pinyin_parser::PinyinParser;
assert_eq!(
    PinyinParser::strict("Ẑāŋ").into_iter().collect::<Vec<_>>(),
    vec!["zhāng"]
)
```

```rust
use pinyin_parser::PinyinParser;
assert_eq!(
    // An apostrophe can come only before an `a`, an `e` or an `o` in strict mode,
    // but allowed here because it's loose    
    PinyinParser::loose("Yīng'guó") 
        .into_iter()
        .collect::<Vec<_>>(),
    vec!["yīng", "guó"]
);
```
