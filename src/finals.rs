use crate::{pinyin_token, VecAndIndex};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NonRhoticFinal {
    A,
    Ai,
    An,
    Ang,
    Ao,
    E,
    Ê,
    Ei,
    En,
    Eng,
    I,
    Ia,
    Ian,
    Iang,
    Iao,
    Ie,
    In,
    Ing,
    Iong,
    Iu,
    Io,
    O,
    Ong,
    Ou,
    U,
    Ua,
    Uai,
    Uan,
    Uang,
    Ue,
    Ui,
    Un,
    Uo,
    Ü,
    Üan,
    Üe,
    Ün,
}

impl NonRhoticFinal {
    #[must_use]
    pub const fn len(self) -> usize {
        use NonRhoticFinal::{
            Ai, An, Ang, Ao, Ei, En, Eng, Ia, Ian, Iang, Iao, Ie, In, Ing, Io, Iong, Iu, Ong, Ou,
            Ua, Uai, Uan, Uang, Ue, Ui, Un, Uo, Üan, Üe, Ün, A, E, I, O, U, Ê, Ü,
        };
        match self {
            A | E | Ê | I | O | U | Ü => 1,
            Ai | An | Ao | Ei | En | Ia | Ie | In | Iu | Io | Ou | Ua | Ue | Ui | Un | Uo | Üe
            | Ün => 2,
            Ang | Eng | Ian | Iao | Ing | Ong | Uai | Uan | Üan => 3,
            Iang | Iong | Uang => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tone {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

macro_rules! toneless {
    ($self_:expr, $ind:expr, $alphabet_pat:pat) => {
        match $self_.vec.get($self_.next_pos + $ind) {
            Some(PinyinToken::Alph(alph)) => {
                matches!(alph.alphabet, $alphabet_pat) && alph.diacritics.is_empty()
            }
            _ => false,
        }
    };

    ($self_:expr, $ind:expr, $alphabet_pat:pat, $diacritic_pat:pat) => {
        match $self_.vec.get($self_.next_pos + $ind) {
            Some(PinyinToken::Alph(alph)) => {
                matches!(alph.alphabet, $alphabet_pat)
                    && matches!(&alph.diacritics[..], &[$diacritic_pat])
            }
            _ => false,
        }
    };
}

macro_rules! tone {
    ($self_:expr, $strict_flag: expr, $ind:expr, $alphabet_pat:pat) => {
        match $self_.vec.get($self_.next_pos + $ind) {
            Some(PinyinToken::Alph(alph)) => {
                if matches!(alph.alphabet, $alphabet_pat) {
                    match &alph.diacritics[..] {
                        &[Diacritic::Macron] => Some(Tone::First),
                        &[Diacritic::Acute] => Some(Tone::Second),
                        &[Diacritic::Hacek] => Some(Tone::Third),
                        &[Diacritic::Breve] => {
                            if $strict_flag {
                                None
                            } else {
                                Some(Tone::Third)
                            }
                        }
                        &[Diacritic::Grave] => Some(Tone::Fourth),
                        &[] => Some(Tone::Fifth),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    };

    ($self_:expr, $strict_flag: expr, $ind:expr, $alphabet_pat:pat, $diacritic_pat:pat) => {
        match $self_.vec.get($self_.next_pos + $ind) {
            Some(PinyinToken::Alph(alph)) => {
                if matches!(alph.alphabet, $alphabet_pat) {
                    match &alph.diacritics[..] {
                        &[$diacritic_pat, Diacritic::Macron] => Some(Tone::First),
                        &[$diacritic_pat, Diacritic::Acute] => Some(Tone::Second),
                        &[$diacritic_pat, Diacritic::Hacek] => Some(Tone::Third),
                        &[$diacritic_pat, Diacritic::Breve] => {
                            if $strict_flag {
                                None
                            } else {
                                Some(Tone::Third)
                            }
                        }
                        &[$diacritic_pat, Diacritic::Grave] => Some(Tone::Fourth),

                        &[Diacritic::Macron, $diacritic_pat] => Some(Tone::First),
                        &[Diacritic::Acute, $diacritic_pat] => Some(Tone::Second),
                        &[Diacritic::Hacek, $diacritic_pat] => Some(Tone::Third),
                        &[Diacritic::Breve, $diacritic_pat] => {
                            if $strict_flag {
                                None
                            } else {
                                Some(Tone::Third)
                            }
                        }
                        &[Diacritic::Grave, $diacritic_pat] => Some(Tone::Fourth),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    };
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Candidate {
    pub ŋ: bool,
    pub fin: NonRhoticFinal,
    pub tone: Tone,
}

pub struct FinalWithTone {
    pub fin: NonRhoticFinal,
    pub tone: Tone,
}

impl std::fmt::Display for FinalWithTone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use unicode_normalization::UnicodeNormalization;
        let (a, c) = match self.fin {
            NonRhoticFinal::A => ("a", ""),
            NonRhoticFinal::Ai => ("a", "i"),
            NonRhoticFinal::An => ("a", "n"),
            NonRhoticFinal::Ang => ("a", "ng"),
            NonRhoticFinal::Ao => ("a", "o"),
            NonRhoticFinal::E => ("e", ""),
            NonRhoticFinal::Ê => ("ê", ""),
            NonRhoticFinal::Ei => ("e", "i"),
            NonRhoticFinal::En => ("e", "n"),
            NonRhoticFinal::Eng => ("e", "ng"),
            NonRhoticFinal::I => ("i", ""),
            NonRhoticFinal::Ia => ("ia", ""),
            NonRhoticFinal::Ian => ("ia", "n"),
            NonRhoticFinal::Iang => ("ia", "ng"),
            NonRhoticFinal::Iao => ("ia", "o"),
            NonRhoticFinal::Ie => ("ie", ""),
            NonRhoticFinal::In => ("i", "n"),
            NonRhoticFinal::Ing => ("i", "ng"),
            NonRhoticFinal::Iong => ("io", "ng"),
            NonRhoticFinal::Iu => ("iu", ""),
            NonRhoticFinal::Io => ("io", ""),
            NonRhoticFinal::O => ("o", ""),
            NonRhoticFinal::Ong => ("o", "ng"),
            NonRhoticFinal::Ou => ("o", "u"),
            NonRhoticFinal::U => ("u", ""),
            NonRhoticFinal::Ua => ("ua", ""),
            NonRhoticFinal::Uai => ("ua", "i"),
            NonRhoticFinal::Uan => ("ua", "n"),
            NonRhoticFinal::Uang => ("ua", "ng"),
            NonRhoticFinal::Ue => ("ue", ""),
            NonRhoticFinal::Ui => ("ui", ""),
            NonRhoticFinal::Un => ("u", "n"),
            NonRhoticFinal::Uo => ("uo", ""),
            NonRhoticFinal::Ü => ("ü", ""),
            NonRhoticFinal::Üan => ("üa", "n"),
            NonRhoticFinal::Üe => ("üe", ""),
            NonRhoticFinal::Ün => ("ü", "n"),
        };

        let b = match self.tone {
            Tone::First => "\u{304}",
            Tone::Second => "\u{301}",
            Tone::Third => "\u{30c}",
            Tone::Fourth => "\u{300}",
            Tone::Fifth => "",
        };

        let ans = format!("{}{}{}", a, b, c);
        let ans = ans.nfc().collect::<String>();
        write!(f, "{}", ans)
    }
}

impl VecAndIndex<pinyin_token::PinyinToken> {
    #[must_use]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)]
    pub fn get_candidates_without_rhotic(&self, strict: bool) -> Vec<Candidate> {
        use pinyin_token::{Alphabet, Diacritic, PinyinToken};
        let mut ans = Vec::new();

        if let Some(tone) = tone!(self, strict, 0, Alphabet::A) {
            ans.push(Candidate {
                ŋ: false,
                fin: NonRhoticFinal::A,
                tone,
            });

            if toneless!(self, 1, Alphabet::I) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ai,
                    tone,
                });
            }

            if toneless!(self, 1, Alphabet::N) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::An,
                    tone,
                })
            }

            if toneless!(self, 1, Alphabet::Ŋ) {
                ans.push(Candidate {
                    ŋ: true,
                    fin: NonRhoticFinal::Ang,
                    tone,
                })
            }

            if toneless!(self, 1, Alphabet::N) && toneless!(self, 2, Alphabet::G) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ang,
                    tone,
                })
            }

            if toneless!(self, 1, Alphabet::O) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ao,
                    tone,
                })
            }
        }

        if let Some(tone) = tone!(self, strict, 0, Alphabet::E, Diacritic::Circumflex) {
            ans.push(Candidate {
                ŋ: false,
                fin: NonRhoticFinal::Ê,
                tone,
            });
        }

        if let Some(tone) = tone!(self, strict, 0, Alphabet::E) {
            ans.push(Candidate {
                ŋ: false,
                fin: NonRhoticFinal::E,
                tone,
            });

            if toneless!(self, 1, Alphabet::I) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ei,
                    tone,
                });
            }

            if toneless!(self, 1, Alphabet::N) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::En,
                    tone,
                })
            }

            if toneless!(self, 1, Alphabet::N) && toneless!(self, 2, Alphabet::G) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Eng,
                    tone,
                })
            }

            if toneless!(self, 1, Alphabet::Ŋ) {
                ans.push(Candidate {
                    ŋ: true,
                    fin: NonRhoticFinal::Eng,
                    tone,
                })
            }
        }

        if let Some(tone) = tone!(self, strict, 0, Alphabet::O) {
            ans.push(Candidate {
                ŋ: false,
                fin: NonRhoticFinal::O,
                tone,
            });

            if toneless!(self, 1, Alphabet::Ŋ) {
                ans.push(Candidate {
                    ŋ: true,
                    fin: NonRhoticFinal::Ong,
                    tone,
                })
            }

            if toneless!(self, 1, Alphabet::N) && toneless!(self, 2, Alphabet::G) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ong,
                    tone,
                })
            }

            if toneless!(self, 1, Alphabet::U) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ou,
                    tone,
                })
            }
        }

        // For I, U and Ü, we must cover both the tone! and toneless!,
        // since the light tone (which is accepted by tone!) is indistinguishable
        // from the toneless.
        if let Some(tone) = tone!(self, strict, 0, Alphabet::I) {
            ans.push(Candidate {
                ŋ: false,
                fin: NonRhoticFinal::I,
                tone,
            });

            if toneless!(self, 1, Alphabet::N) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::In,
                    tone,
                });
            }

            if toneless!(self, 1, Alphabet::N) && toneless!(self, 2, Alphabet::G) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ing,
                    tone,
                })
            }

            if toneless!(self, 1, Alphabet::Ŋ) {
                ans.push(Candidate {
                    ŋ: true,
                    fin: NonRhoticFinal::Ing,
                    tone,
                })
            }
        }

        if toneless!(self, 0, Alphabet::I) {
            // -ia...
            if let Some(tone) = tone!(self, strict, 1, Alphabet::A) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ia,
                    tone,
                });

                if toneless!(self, 2, Alphabet::N) {
                    ans.push(Candidate {
                        ŋ: false,
                        fin: NonRhoticFinal::Ian,
                        tone,
                    })
                }

                if toneless!(self, 2, Alphabet::N) && toneless!(self, 3, Alphabet::G) {
                    ans.push(Candidate {
                        ŋ: false,
                        fin: NonRhoticFinal::Iang,
                        tone,
                    })
                }

                if toneless!(self, 2, Alphabet::Ŋ) {
                    ans.push(Candidate {
                        ŋ: true,
                        fin: NonRhoticFinal::Iang,
                        tone,
                    })
                }

                if toneless!(self, 2, Alphabet::O) {
                    ans.push(Candidate {
                        ŋ: false,
                        fin: NonRhoticFinal::Iao,
                        tone,
                    })
                }
            } // end -ia..

            if let Some(tone) = tone!(self, strict, 1, Alphabet::E) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ie,
                    tone,
                })
            }

            if let Some(tone) = tone!(self, strict, 1, Alphabet::U) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Iu,
                    tone,
                })
            }

            if let Some(tone) = tone!(self, strict, 1, Alphabet::O) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Io,
                    tone,
                });
                if toneless!(self, 2, Alphabet::N) && toneless!(self, 3, Alphabet::G) {
                    ans.push(Candidate {
                        ŋ: false,
                        fin: NonRhoticFinal::Iong,
                        tone,
                    })
                }

                if toneless!(self, 2, Alphabet::Ŋ) {
                    ans.push(Candidate {
                        ŋ: true,
                        fin: NonRhoticFinal::Iong,
                        tone,
                    })
                }
            }
        }

        if let Some(tone) = tone!(self, strict, 0, Alphabet::U) {
            ans.push(Candidate {
                ŋ: false,
                fin: NonRhoticFinal::U,
                tone,
            });

            if toneless!(self, 1, Alphabet::N) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Un,
                    tone,
                })
            }
        }

        if toneless!(self, 0, Alphabet::U) {
            // -ua..
            if let Some(tone) = tone!(self, strict, 1, Alphabet::A) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ua,
                    tone,
                });

                if toneless!(self, 2, Alphabet::I) {
                    ans.push(Candidate {
                        ŋ: false,
                        fin: NonRhoticFinal::Uai,
                        tone,
                    })
                }

                if toneless!(self, 2, Alphabet::N) {
                    ans.push(Candidate {
                        ŋ: false,
                        fin: NonRhoticFinal::Uan,
                        tone,
                    })
                }

                if toneless!(self, 2, Alphabet::N) && toneless!(self, 3, Alphabet::G) {
                    ans.push(Candidate {
                        ŋ: false,
                        fin: NonRhoticFinal::Uang,
                        tone,
                    })
                }

                if toneless!(self, 2, Alphabet::Ŋ) {
                    ans.push(Candidate {
                        ŋ: true,
                        fin: NonRhoticFinal::Uang,
                        tone,
                    })
                }
            } // end -ua..

            if let Some(tone) = tone!(self, strict, 1, Alphabet::E) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ue,
                    tone,
                });
            }

            if let Some(tone) = tone!(self, strict, 1, Alphabet::I) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ui,
                    tone,
                });
            }

            if let Some(tone) = tone!(self, strict, 1, Alphabet::O) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Uo,
                    tone,
                });
            }
        }

        if let Some(tone) = tone!(self, strict, 0, Alphabet::U, Diacritic::Umlaut) {
            ans.push(Candidate {
                ŋ: false,
                fin: NonRhoticFinal::Ü,
                tone,
            });

            if toneless!(self, 1, Alphabet::N) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Ün,
                    tone,
                })
            }
        }
        if toneless!(self, 0, Alphabet::U, Diacritic::Umlaut) {
            if let Some(tone) = tone!(self, strict, 1, Alphabet::A) {
                if toneless!(self, 2, Alphabet::N) {
                    ans.push(Candidate {
                        ŋ: false,
                        fin: NonRhoticFinal::Üan,
                        tone,
                    })
                }
            }

            if let Some(tone) = tone!(self, strict, 1, Alphabet::E) {
                ans.push(Candidate {
                    ŋ: false,
                    fin: NonRhoticFinal::Üe,
                    tone,
                })
            }
        }

        ans
    }
}
