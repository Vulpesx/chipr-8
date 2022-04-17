
pub struct Keyboard<const SIZE: usize, I, O> {
    key_set: [char; SIZE],
    p: fn(&Self, [char; SIZE], I) -> O,
}

impl<const SIZE: usize, I, O> Keyboard<SIZE, I, O> {
    pub fn new(keys: [char; SIZE], p: fn(&Self, [char; SIZE], I) -> O) -> Keyboard<SIZE, I, O> {
        Keyboard {
            key_set: keys,
            p,
        }
    }

    pub fn is_char_valid(&self, c: char) -> bool {
        self.key_set.contains(&c)
    }

    pub fn is_str_valid(&self, s: &str) -> bool {
        let s: Vec<char> = s.chars().collect();
        for c in s {
            if !self.is_char_valid(c) {
                return false;
            }
        }

        true
    }

    pub fn parse_input(&self, i: I) -> O {
        (self.p)(self, self.key_set, i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum InputReq {
        HEX,
        TO_INT,
    }
    struct StrInput {
        input: String,
        req: InputReq,
    }

    impl StrInput {
        pub fn new(i: &str, r: InputReq) -> StrInput {
            StrInput {
                input: String::from(i),
                req: r,
            }
        }
    }

    fn test_keyboard() -> Keyboard<16, StrInput, u8> {
        Keyboard::new(
            [
            '1','2','3','C',
            '4','5','6','D',
            '7','8','9','E',
            'A','0','B','F',
            ],
            |_, _, i: StrInput| {
                i.input.parse::<u8>().unwrap()
            })
    }

    #[test]
    fn test_is_char_valid() {
        let k = test_keyboard();
        assert!(k.is_char_valid('F'));
        assert!(!k.is_char_valid('f'))
    }

    #[test]
    fn test_is_str_valid() {
        let k = test_keyboard();
        assert!(k.is_str_valid("15FE"));
        assert!(!k.is_str_valid("ljlkjlk"));
    }

    #[test]
    fn test_parse_input() {
        let k = test_keyboard();
        let i = k.parse_input(StrInput::new("12", InputReq::TO_INT));
        assert_eq!(i, 12);
    }

    #[test]
    fn test_impl() {
        

        let k = Keyboard::new(['5'; 5], |_, _, i| {});

        k.is_char_valid('5');
        k.is_str_valid("555");
        k.parse_input(5);
    }
}
