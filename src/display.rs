use crate::mem::MemBuf;

pub struct Display<const SIZE: usize> {
    w: usize,
    h: usize,
    s: usize,
    b: [u8; SIZE]
}

impl<const SIZE: usize> Display<SIZE> {
    pub fn new(w: usize, h: usize, s: usize) -> Display<SIZE> {
        Display { w, h, s, b: [0; SIZE] }
    }

    pub fn update<const S: usize>(&mut self, m: &MemBuf<S>) {
        let m = m.get_buf();
        for i in 0..self.b.len() {
           self.b[i] = m[i + self.s];
        }
    }

    pub fn as_bytes(&self) -> [u8; SIZE] {
        self.b
    }

    pub fn as_str(&self) -> String {
        self.format(" ", "*")
    }

    pub fn format(&self, f: &str, t: &str) -> String {
        let mut s = String::new();
        for b  in self.b {
            for i in 0..8 {
                if b >> i & 1 != 0 {
                    s = format!("{}{}", t, s);
                } else {
                    s = format!("{}{}", f, s);
                }
            }
            s = format!("{}\n", s);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::MemBuf;

    #[test]
    fn test_as_bytes() {
        let m = MemBuf::<2>::new([1; 2], 0);
        let mut d = Display::new(8, 1, 0);
        d.update(&m);
        assert_eq!([1], d.as_bytes());
    }

    #[test]
    fn test_as_str() {
        let m = MemBuf::<2>::new([1; 2], 0);
        let mut d = Display::<1>::new(8, 1, 0);
        d.update(&m);
        assert_eq!("       *\n", d.as_str());
    }

    #[test]
    fn test_format() {
        let m = MemBuf::<2>::new([1;2], 0);
        let mut d = Display::<1>::new(8, 1, 0);
        d.update(&m);
        assert_eq!("       -\n", d.format(" ", "-"));
    }

    #[test]
    fn test_impl() {
        let m = MemBuf::default();
        let mut d = Display::<32>::new(64, 32, 5);
        d.update(&m);
        d.as_str();
        d.as_bytes();
        d.format(" ", "*");
    }
}
