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
    ("=A=l", "Allylouia"),
    ("I=y=c", "Iycouc"),
    ("P=,=c", "Pi`,rictoc"),
    // efnouti is a special abbreviation
    // It is neither a special character nor intersperesd with = or \u{305} and there's no jemkin above it
    // This is due to cultural convention, where the word God never has anything above it
    // This makes it difficult to tell if the string V] is the beginning of a word or an abbreviation
    ("V] ", "Vnou] "),
    (" V].", " Vnou]."),
    (" V]@", " Vnou]@"),
];

pub trait Converter {
    fn convert1(&self, c: char) -> char;
    fn convert(&self, s: String) -> String;
}
mod tests {

    #[test]
    fn charmaps_cs_unique() {
        let mut charset = std::collections::HashSet::new();
        for (cs, _) in super::CHARMAP.iter() {
            assert!(
                charset.insert(cs),
                "Coptic Standard character mapped to multiple unicode characters: {}",
                cs
            );
        }
    }

    #[test]
    fn charmaps_u_unique() {
        let mut charset = std::collections::HashSet::new();
        for (_, u) in super::CHARMAP.iter() {
            assert!(
                charset.insert(u),
                "Unicode character mapped to multiple Coptic Standard characters: {}",
                u
            );
        }
    }

    #[test]
    fn abbreviations_abbreviation_unique() {
        let mut charset = std::collections::HashSet::new();
        for (cs, _) in super::ABBREVIATIONS.iter() {
            assert!(
                charset.insert(cs),
                "Abbreviation mapped to multiple words: {}",
                cs
            );
        }
    }

    #[test]
    fn abbreviations_word_unique() {
        let mut charset = std::collections::HashSet::new();
        for (_, word) in super::ABBREVIATIONS.iter() {
            assert!(
                charset.insert(word),
                "Word mapped to multiple abbreviations: {}",
                word
            );
        }
    }
}
