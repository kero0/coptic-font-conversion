use crate::data::*;
use std::convert::identity;

fn unabbreviate(s: String) -> String {
        let mut s = s;
        ABBREVIATIONS
                .iter()
                .for_each(|(abbreviation, full_name)| s = s.replace(abbreviation, full_name));
        s
}

fn abbreviate(s: String) -> String {
        let mut s = s;
        ABBREVIATIONS
                .iter()
                .for_each(|(abbreviation, full_name)| s = s.replace(full_name, abbreviation));
        s
}

fn convert(s: String, converter: fn(char) -> char) -> String {
        s.chars().map(converter).collect::<String>()
}

pub fn mk_converter(
        conversion_direction: ConversionType,
        abbreviation_handling: AbbreviationHandling,
) -> Box<dyn Fn(String) -> String> {
        let abbreviator = match abbreviation_handling {
                AbbreviationHandling::Unabbreviate => unabbreviate,
                AbbreviationHandling::Abbreviate => abbreviate,
                AbbreviationHandling::Preserve => identity,
        };

        return match conversion_direction {
                ConversionType::UnicodeToCopticStandard => {
                        let swap_abbreviator = match abbreviation_handling {
                                AbbreviationHandling::Unabbreviate => identity,
                                _ => {
                                        |s: String| -> String {
                                                let mut v: Vec<char> = Vec::with_capacity(s.len());
                                                for c in s.chars() {
                                                        if c == '\u{305}' {
                                                                let c = v.pop().expect("Malformed abbreviation detected");
                                                                v.push('=');
                                                                v.push(c);
                                                        } else {
                                                                v.push(c);
                                                        }
                                                }
                                                v.iter().collect::<String>()
                                        }
                                }
                        };
                        Box::new(move |s: String| {
                                let s = swap_abbreviator(s);
                                let s = convert(s, |c: char| {
                                        CHARMAP.iter()
                                                .find(|&&(_, c1)| c1 == c)
                                                .map(|&(c1, _)| c1)
                                                .unwrap_or(c)
                                });
                                abbreviator(s)
                        })
                }
                ConversionType::CopticStandardToUnicode => {
                        let swap_abbreviator = match abbreviation_handling {
                                AbbreviationHandling::Unabbreviate => std::convert::identity,
                                _ => |s: String| -> String {
                                        let mut v = Vec::with_capacity(s.len());
                                        let mut last_was_equals = false;
                                        for c in s.chars() {
                                                if last_was_equals {
                                                        v.push(c);
                                                        v.push('\u{305}');
                                                        last_was_equals = false;
                                                } else if c == '=' {
                                                        last_was_equals = true;
                                                } else {
                                                        v.push(c);
                                                }
                                        }
                                        v.iter().collect::<String>()
                                },
                        };
                        Box::new(move |s: String| {
                                let s = abbreviator(s);
                                let s = swap_abbreviator(s);

                                convert(s, |c: char| {
                                        CHARMAP.iter()
                                                .find(|&&(c1, _)| c1 == c)
                                                .map(|&(_, c2)| c2)
                                                .unwrap_or(c)
                                })
                        })
                }
        };
}

#[cfg(test)]
mod tests {
        use super::*;
        use std::str::FromStr;

        #[test]
        fn e2c() {
                let converter = mk_converter(
                        ConversionType::CopticStandardToUnicode,
                        AbbreviationHandling::Preserve,
                );
                assert_eq!(converter(
                        String::from_str(
                                "<ere na=o=c `nio]@ `n`apoctoloc@ ,ere nima;ytyc@ `nte pen=o=c I=y=c P=,=c.")
                                .unwrap()),
                           "Ⲭⲉⲣⲉ ⲛⲁⲟ̅ⲥ̅ ⳿ⲛⲓⲟϯ: ⳿ⲛ⳿ⲁⲡⲟⲥⲧⲟⲗⲟⲥ: ⲭⲉⲣⲉ ⲛⲓⲙⲁⲑⲏⲧⲏⲥ: ⳿ⲛⲧⲉ ⲡⲉⲛⲟ̅ⲥ̅ Ⲓⲏ̅ⲥ̅ Ⲡⲭ̅ⲥ̅."
                );
        }
        #[test]
        fn c2e() {
                let converter = mk_converter(
                        ConversionType::UnicodeToCopticStandard,
                        AbbreviationHandling::Preserve,
                );
                assert_eq!(
                        "<ere na=o=c `nio]@ `n`apoctoloc@ ,ere nima;ytyc@ `nte pen=o=c I=y=c P=,=c.",
                        converter(String::from_str(
                                "Ⲭⲉⲣⲉ ⲛⲁⲟ̅ⲥ̅ ⳿ⲛⲓⲟϯ: ⳿ⲛ⳿ⲁⲡⲟⲥⲧⲟⲗⲟⲥ: ⲭⲉⲣⲉ ⲛⲓⲙⲁⲑⲏⲧⲏⲥ: ⳿ⲛⲧⲉ ⲡⲉⲛⲟ̅ⲥ̅ Ⲓⲏ̅ⲥ̅ Ⲡⲭ̅ⲥ̅.").unwrap())
                );
        }

        #[test]
        fn e2c_abbreviate() {
                let converter = mk_converter(
                        ConversionType::CopticStandardToUnicode,
                        AbbreviationHandling::Unabbreviate,
                );
                assert_eq!(converter(String::from_str("pen=o=c").unwrap()), "ⲡⲉⲛϭⲟⲓⲥ");
                assert_eq!(converter(String::from_str("I=y=c").unwrap()), "Ⲓⲏⲥⲟⲩⲥ");
                assert_eq!(converter(String::from_str("P=,=c").unwrap()), "Ⲡⲓ⳿ⲭⲣⲓⲥⲧⲟⲥ");
                assert_eq!(
                        converter(String::from_str("pen=o=c I=y=c P=,=c").unwrap()),
                        "ⲡⲉⲛϭⲟⲓⲥ Ⲓⲏⲥⲟⲩⲥ Ⲡⲓ⳿ⲭⲣⲓⲥⲧⲟⲥ"
                );
        }
}
