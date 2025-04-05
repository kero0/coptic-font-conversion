use crate::data::*;

fn unabbreviate(s: String) -> String {
    ABBREVIATIONS
        .iter()
        .fold(s, |s, (abbreviation, full_name)| {
            s.replace(abbreviation, full_name)
        })
}

fn abbreviate(s: String) -> String {
    ABBREVIATIONS
        .iter()
        .fold(s, |s, (abbreviation, full_name)| {
            s.replace(full_name, abbreviation)
        })
}

impl Converter for ConversionType {
    fn convert1(&self, c: char) -> char {
        match self {
            ConversionType::CopticStandardToUnicode => CHARMAP
                .iter()
                .find(|(k, _)| *k == c)
                .map(|(_, v)| *v)
                .unwrap_or(c),
            ConversionType::UnicodeToCopticStandard => CHARMAP
                .iter()
                .find(|(_, v)| *v == c)
                .map(|(k, _)| *k)
                .unwrap_or(c),
        }
    }
    fn convert(&self, s: String) -> String {
        match self {
            ConversionType::CopticStandardToUnicode => {
                let mut v: Vec<char> = Vec::with_capacity(s.len());
                let mut eq: bool = false;
                for c in s.chars() {
                    if c == '=' {
                        eq = true;
                    } else if eq {
                        v.push(self.convert1(c));
                        v.push('\u{305}');
                        eq = false;
                    } else {
                        v.push(self.convert1(c));
                    }
                }
                v.into_iter().collect()
            }

            ConversionType::UnicodeToCopticStandard => {
                let mut v: Vec<char> = Vec::with_capacity(s.len());
                let mut eq: bool = false;
                for c in s.chars() {
                    if c == '\u{305}' {
                        eq = true;
                    } else if eq {
                        let c1 = v
                            .pop()
                            .expect(format!("Malformed abbreviation in string: {}", s).as_str());
                        v.push('=');
                        v.push(c1);
                        v.push(self.convert1(c));
                        eq = false;
                    } else {
                        v.push(self.convert1(c));
                    }
                }
                v.into_iter().collect()
            }
        }
    }
}

pub fn mk_converter(
    direction: ConversionType,
    abbreviation_handling: AbbreviationHandling,
) -> Box<dyn Fn(String) -> String> {
    let abbreviator = match abbreviation_handling {
        AbbreviationHandling::Abbreviate => abbreviate,
        AbbreviationHandling::Unabbreviate => unabbreviate,
        AbbreviationHandling::Preserve => std::convert::identity,
    };

    match direction {
        ConversionType::CopticStandardToUnicode => {
            Box::new(move |s| abbreviator(direction.convert(s)))
        }
        ConversionType::UnicodeToCopticStandard => {
            Box::new(move |s| direction.convert(abbreviator(s)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    macro_rules! test_case {
        ($name:ident, $conversion_type:expr, $abbreviation_handling:expr, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let converter = mk_converter($conversion_type, $abbreviation_handling);
                assert_eq!(converter(String::from_str($input).unwrap()), $expected)
            }
        };
    }

    test_case!(
        e2c,
        ConversionType::CopticStandardToUnicode,
        AbbreviationHandling::Preserve,
        "<ere na=o=c `nio]@ `n`apoctoloc@ ,ere nima;ytyc@ `nte pen=o=c I=y=c P=,=c.",
        "Ⲭⲉⲣⲉ ⲛⲁⲟ̅ⲥ̅ ⳿ⲛⲓⲟϯ: ⳿ⲛ⳿ⲁⲡⲟⲥⲧⲟⲗⲟⲥ: ⲭⲉⲣⲉ ⲛⲓⲙⲁⲑⲏⲧⲏⲥ: ⳿ⲛⲧⲉ ⲡⲉⲛⲟ̅ⲥ̅ Ⲓⲏ̅ⲥ̅ Ⲡⲭ̅ⲥ̅."
    );

    test_case!(
        c2e,
        ConversionType::UnicodeToCopticStandard,
        AbbreviationHandling::Preserve,
        "Ⲭⲉⲣⲉ ⲛⲁⲟ̅ⲥ̅ ⳿ⲛⲓⲟϯ: ⳿ⲛ⳿ⲁⲡⲟⲥⲧⲟⲗⲟⲥ: ⲭⲉⲣⲉ ⲛⲓⲙⲁⲑⲏⲧⲏⲥ: ⳿ⲛⲧⲉ ⲡⲉⲛⲟ̅ⲥ̅ Ⲓⲏ̅ⲥ̅ Ⲡⲭ̅ⲥ̅.",
        "<ere na=o=c `nio]@ `n`apoctoloc@ ,ere nima;ytyc@ `nte pen=o=c I=y=c P=,=c."
    );

    test_case!(
        e2c_abbreviate,
        ConversionType::CopticStandardToUnicode,
        AbbreviationHandling::Abbreviate,
        "<ere na=o=c `nio]@ `n`apoctoloc@ ,ere nima;ytyc@ `nte pen=o=c I=y=c P=,=c.",
        "Ⲭⲉⲣⲉ ⲛⲁⲟ̅ⲥ̅ ⳿ⲛⲓⲟϯ: ⳿ⲛ⳿ⲁⲡⲟⲥⲧⲟⲗⲟⲥ: ⲭⲉⲣⲉ ⲛⲓⲙⲁⲑⲏⲧⲏⲥ: ⳿ⲛⲧⲉ ⲡⲉⲛⲟ̅ⲥ̅ Ⲓⲏ̅ⲥ̅ Ⲡⲭ̅ⲥ̅."
    );

    test_case!(
        c2e_abbreviate,
        ConversionType::UnicodeToCopticStandard,
        AbbreviationHandling::Abbreviate,
        "Ⲭⲉⲣⲉ ⲛⲁⲟ̅ⲥ̅ ⳿ⲛⲓⲟϯ: ⳿ⲛ⳿ⲁⲡⲟⲥⲧⲟⲗⲟⲥ: ⲭⲉⲣⲉ ⲛⲓⲙⲁⲑⲏⲧⲏⲥ: ⳿ⲛⲧⲉ ⲡⲉⲛⲟ̅ⲥ̅ Ⲓⲏ̅ⲥ̅ Ⲡⲭ̅ⲥ̅.",
        "<ere na=o=c `nio]@ `n`apoctoloc@ ,ere nima;ytyc@ `nte pen=o=c I=y=c P=,=c."
    );
}
