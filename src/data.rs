use clap::ArgEnum;
use std::clone::Clone;

pub const CHARMAP: [(char, char); 65] = [
    ('a', 'ⲁ'),
    ('b', 'ⲃ'),
    ('c', 'ⲥ'),
    ('d', 'ⲇ'),
    ('e', 'ⲉ'),
    ('f', 'ϥ'),
    ('g', 'ⲅ'),
    ('h', 'ϩ'),
    ('i', 'ⲓ'),
    ('j', 'ϫ'),
    ('k', 'ⲕ'),
    ('l', 'ⲗ'),
    ('m', 'ⲙ'),
    ('n', 'ⲛ'),
    ('o', 'ⲟ'),
    ('p', 'ⲡ'),
    ('q', 'ϧ'),
    ('r', 'ⲣ'),
    ('s', 'ϣ'),
    ('t', 'ⲧ'),
    ('u', 'ⲩ'),
    ('v', 'ⲫ'),
    ('w', 'ⲱ'),
    ('x', 'ⲝ'),
    ('y', 'ⲏ'),
    ('z', 'ⲍ'),
    ('A', 'Ⲁ'),
    ('B', 'Ⲃ'),
    ('C', 'Ⲥ'),
    ('D', 'Ⲇ'),
    ('E', 'Ⲉ'),
    ('F', 'Ϥ'),
    ('G', 'Ⲅ'),
    ('H', 'Ϩ'),
    ('I', 'Ⲓ'),
    ('J', 'Ϫ'),
    ('K', 'Ⲕ'),
    ('L', 'Ⲗ'),
    ('M', 'Ⲙ'),
    ('N', 'Ⲛ'),
    ('O', 'Ⲟ'),
    ('P', 'Ⲡ'),
    ('Q', 'Ϧ'),
    ('R', 'Ⲣ'),
    ('S', 'Ϣ'),
    ('T', 'Ⲧ'),
    ('U', 'Ⲩ'),
    ('V', 'Ⲫ'),
    ('W', 'Ⲱ'),
    ('X', 'Ⲝ'),
    ('Y', 'Ⲏ'),
    ('Z', 'Ⲍ'),
    (';', 'ⲑ'),
    (':', 'Ⲑ'),
    (',', 'ⲭ'),
    ('<', 'Ⲭ'),
    ('\'', 'ⲯ'),
    ('"', 'Ⲯ'),
    ('[', 'ϭ'),
    ('{', 'Ϭ'),
    (']', 'ϯ'),
    ('}', 'Ϯ'),
    ('`', '⳿'),
    ('@', ':'),
    ('^', 'ⲋ'),
];

pub const ABBREVIATIONS: [(&'static str, &'static str); 9] = [
    ("¥", "<rictoc"),
    ("¢", "Marturoc"),
    ("=o=c", "[oic"),
    ("V] ", "Vnou] "),
    (" V] ", " Vnou]"),
    ("=A=l", "Allylouia"),
    ("=o=c", "[oic"),
    ("I=y=c", "Iycouc"),
    ("P=,=c", "Pi`,rictoc"),
];
#[derive(ArgEnum)]
pub enum ConversionType {
    CopticStandardToUnicode,
    UnicodeToCopticStandard,
}

#[derive(ArgEnum)]
pub enum AbbreviationHandling {
    Preserve,
    Unabbreviate,
    Abbreviate,
}

impl Clone for ConversionType {
    fn clone(&self) -> Self {
        match self {
            ConversionType::CopticStandardToUnicode => ConversionType::CopticStandardToUnicode,
            ConversionType::UnicodeToCopticStandard => ConversionType::UnicodeToCopticStandard,
        }
    }
}

impl Clone for AbbreviationHandling {
    fn clone(&self) -> Self {
        match self {
            AbbreviationHandling::Preserve => AbbreviationHandling::Preserve,
            AbbreviationHandling::Unabbreviate => AbbreviationHandling::Unabbreviate,
            AbbreviationHandling::Abbreviate => AbbreviationHandling::Abbreviate,
        }
    }
}
