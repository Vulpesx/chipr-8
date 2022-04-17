use std::{error::Error, num::ParseIntError};

const KEYS: [char; 16] = [
        '1','2','3','C',
        '4','5','6','D',
        '7','8','9','E',
        'A','0','B','F',];

#[derive(Debug)]
pub enum HexError {
    HexParseIntErr(ParseIntError),
    InvalidHex,
}


pub fn valid_char(c: char) -> bool {
    KEYS.contains(&c)
}

pub fn valid_str(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    for i in 0..c.len() {
        if !valid_char(c[i]) {
            return false;
        }
    }
    s.len() <= 3
}

pub fn from_hex(s: &str) -> Result<u16, HexError> {
    let s = s.to_uppercase();
    if !valid_str(&s) {
        return Err(HexError::InvalidHex);
    }
    let c: Vec<char> = s.chars().collect();
    let mut b: [u16; 3] = [0; 3];
    let mut s: u16 = 0;
    for i in 0..c.len() {
        if c[i].is_numeric() {
            let n = String::from(c[i]).parse::<u16>();
            if n.is_err() { return Err(HexError::HexParseIntErr(n.unwrap_err()))}
            b[i] = n.unwrap();
        } else {
            b[i] = match c[i] {
                'A' => 10,
                'B' => 11,
                'C' => 12,
                'D' => 13,
                'E' => 14,
                'F' => 15,
                _ => return Err(HexError::InvalidHex),
            }
        }
        s += b[i] * (16 * (16 * i as u16));
    }
    Ok(s)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_char() {
        assert!(valid_char('f'));
        assert!(!valid_char('h'));
    }

    #[test]
    fn test_valid_str() {
        assert!(valid_str("fff"));
        assert!(!valid_str("ffff"));
        assert!(!valid_str("ffh"));
    }

    #[test]
    fn test_from_hex() {
        assert_eq!(512, from_hex("200").unwrap());
        assert!(from_hex("hi").is_err());
    }

    #[test]
    fn test_impl() {
        valid_char('3');
        valid_str("1b9");
        from_hex("fff");
    }
}
