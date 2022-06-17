use regex::Regex;
fn _e2c(c: char) -> char {
    return match c {
        'a' => 'ⲁ',
        'b' => 'ⲃ',
        'c' => 'ⲥ',
        'd' => 'ⲇ',
        'e' => 'ⲉ',
        'f' => 'ϥ',
        'g' => 'ⲅ',
        'h' => 'ϩ',
        'i' => 'ⲓ',
        'j' => 'ϫ',
        'k' => 'ⲕ',
        'l' => 'ⲗ',
        'm' => 'ⲙ',
        'n' => 'ⲛ',
        'o' => 'ⲟ',
        'p' => 'ⲡ',
        'q' => 'ϧ',
        'r' => 'ⲣ',
        's' => 'ϣ',
        't' => 'ⲧ',
        'u' => 'ⲩ',
        'v' => 'ⲫ',
        'w' => 'ⲱ',
        'x' => 'ⲝ',
        'y' => 'ⲏ',
        'z' => 'ⲍ',
        'A' => 'Ⲁ',
        'B' => 'Ⲃ',
        'C' => 'Ⲥ',
        'D' => 'Ⲇ',
        'E' => 'Ⲉ',
        'F' => 'Ϥ',
        'G' => 'Ⲅ',
        'H' => 'Ϩ',
        'I' => 'Ⲓ',
        'J' => 'Ϫ',
        'K' => 'Ⲕ',
        'L' => 'Ⲗ',
        'M' => 'Ⲙ',
        'N' => 'Ⲛ',
        'O' => 'Ⲟ',
        'P' => 'Ⲡ',
        'Q' => 'Ϧ',
        'R' => 'Ⲣ',
        'S' => 'Ϣ',
        'T' => 'Ⲧ',
        'U' => 'Ⲩ',
        'V' => 'Ⲫ',
        'W' => 'Ⲱ',
        'X' => 'Ⲝ',
        'Y' => 'Ⲏ',
        'Z' => 'Ⲍ',
        ';' => 'ⲑ',
        ':' => 'Ⲑ',
        ',' => 'ⲭ',
        '<' => 'Ⲭ',
        '\'' => 'ⲯ',
        '"' => 'Ⲯ',
        '[' => 'ϭ',
        '{' => 'Ϭ',
        ']' => 'ϯ',
        '}' => 'Ϯ',
        '`' => '⳿',
        '@' => ':',
        '^' => 'ⲋ',
        _ => c,
    };
}
fn _c2e(c: char) -> char {
    return match c {
        'ⲁ' => 'a',
        'ⲃ' => 'b',
        'ⲥ' => 'c',
        'ⲇ' => 'd',
        'ⲉ' => 'e',
        'ϥ' => 'f',
        'ⲅ' => 'g',
        'ϩ' => 'h',
        'ⲓ' => 'i',
        'ϫ' => 'j',
        'ⲕ' => 'k',
        'ⲗ' => 'l',
        'ⲙ' => 'm',
        'ⲛ' => 'n',
        'ⲟ' => 'o',
        'ⲡ' => 'p',
        'ϧ' => 'q',
        'ⲣ' => 'r',
        'ϣ' => 's',
        'ⲧ' => 't',
        'ⲩ' => 'u',
        'ⲫ' => 'v',
        'ⲱ' => 'w',
        'ⲝ' => 'x',
        'ⲏ' => 'y',
        'ⲍ' => 'z',
        'Ⲁ' => 'A',
        'Ⲃ' => 'B',
        'Ⲥ' => 'C',
        'Ⲇ' => 'D',
        'Ⲉ' => 'E',
        'Ϥ' => 'F',
        'Ⲅ' => 'G',
        'Ϩ' => 'H',
        'Ⲓ' => 'I',
        'Ϫ' => 'J',
        'Ⲕ' => 'K',
        'Ⲗ' => 'L',
        'Ⲙ' => 'M',
        'Ⲛ' => 'N',
        'Ⲟ' => 'O',
        'Ⲡ' => 'P',
        'Ϧ' => 'Q',
        'Ⲣ' => 'R',
        'Ϣ' => 'S',
        'Ⲧ' => 'T',
        'Ⲩ' => 'U',
        'Ⲫ' => 'V',
        'Ⲱ' => 'W',
        'Ⲝ' => 'X',
        'Ⲏ' => 'Y',
        'Ⲍ' => 'Z',
        'ⲑ' => ';',
        'Ⲑ' => ':',
        'ⲭ' => ',',
        'Ⲭ' => '<',
        'ⲯ' => '\'',
        'Ⲯ' => '"',
        'ϭ' => '[',
        'Ϭ' => '{',
        'ϯ' => ']',
        'Ϯ' => '}',
        '⳿' => '`',
        ':' => '@',
        'ⲋ' => '^',
        _ => c,
    };
}

fn convert(s: String, converter: fn(char) -> char) -> String {
    return s.chars().map(converter).collect::<String>();
}
pub fn e2c(s: String) -> String {
    let rs: Regex = Regex::new(r"=(.)").unwrap();
    return convert(
        rs.replace_all(&s, |caps: &regex::Captures| {
            format!("{}{}", &caps[1], '\u{305}')
        })
        .to_string(),
        _e2c,
    );
}

pub fn c2e(s: String) -> String {
    let rs: Regex = Regex::new(r"(.)\u{305}").unwrap();
    return convert(
        rs.replace_all(&s, |caps: &regex::Captures| format!("{}{}", '=', &caps[1]))
            .to_string(),
        _c2e,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_e2c() {
        assert_eq!(
            e2c(String::from_str(
                "<ere na=o=c `nio]@ `n`apoctoloc@ ,ere nima;ytyc@ `nte pen=o=c I=y=c P=,=c."
            )
            .unwrap()),
            "Ⲭⲉⲣⲉ ⲛⲁⲟ̅ⲥ̅ ⳿ⲛⲓⲟϯ: ⳿ⲛ⳿ⲁⲡⲟⲥⲧⲟⲗⲟⲥ: ⲭⲉⲣⲉ ⲛⲓⲙⲁⲑⲏⲧⲏⲥ: ⳿ⲛⲧⲉ ⲡⲉⲛⲟ̅ⲥ̅ Ⲓⲏ̅ⲥ̅ Ⲡⲭ̅ⲥ̅."
        );
    }
    #[test]
    fn test_c2e() {
        assert_eq!(
            c2e(String::from_str(
                "Ⲭⲉⲣⲉ ⲛⲁⲟ̅ⲥ̅ ⳿ⲛⲓⲟϯ: ⳿ⲛ⳿ⲁⲡⲟⲥⲧⲟⲗⲟⲥ: ⲭⲉⲣⲉ ⲛⲓⲙⲁⲑⲏⲧⲏⲥ: ⳿ⲛⲧⲉ ⲡⲉⲛⲟ̅ⲥ̅ Ⲓⲏ̅ⲥ̅ Ⲡⲭ̅ⲥ̅."
            )
            .unwrap()),
            "<ere na=o=c `nio]@ `n`apoctoloc@ ,ere nima;ytyc@ `nte pen=o=c I=y=c P=,=c."
        );
    }
}
