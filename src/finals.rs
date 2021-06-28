use crate::*;
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
    pub fn len(self) -> usize {
        use NonRhoticFinal::*;
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
        match $self_.vec.get($self_.next_pos + 1) {
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
        match $self_.vec.get($self_.next_pos + 1) {
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
                        &[$diacritic_pat] => Some(Tone::Fifth),
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

pub struct Candidate {
    ŋ: bool,
    fin: NonRhoticFinal,
    tone: Tone,
}

impl VecAndIndex<pinyin_token::PinyinToken> {
    pub fn get_candidates_without_rhotic(&self, strict: bool) -> Vec<Candidate> {
        use pinyin_token::*;
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
