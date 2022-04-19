use crate::mem::MemBuf;

/// 8 bit registry
/// meant to be used in conjuction with `MemBuf`
pub struct Reg8 {
    v: u8,
    i: usize,
}

/// 16 bit registry
/// meant to be used in conjuction with `MemBuf`
pub struct Reg16 {
    v: u16,
    i: [usize; 2],
}

impl Reg8 {
    /// construct a registry with an index to
    /// location in `MemBuf` and a default value
    pub fn new(i: usize, v: u8) -> Reg8 {
        Reg8 { v, i }
    }

    /// updates registry from `MemBuf`
    pub fn get<const S: usize>(&mut self, m: &MemBuf<S>) {
        self.v = m.get(self.i);
    }

    /// gets bit at index i
    pub fn get_bit(&self, i: usize) -> bool {
        if i >= 8 {
            panic!("index out of bounds: {}/{}", i, 8);
        }
        self.v & (1 << i) != 0
    }

    /// set `MemBuf` value for stored indexes
    pub fn set<const S: usize>(&self, m: &mut MemBuf<S>) { m.set(self.i, self.v);
    }
    
    /// sets a bit at index i
    pub fn set_bit(&mut self, i: usize, b: bool) {
        if i >= 8 {
            panic!("index out of bounds: {}/{}", i, 8);
        }
        if b {
            self.v = self.v | (1 << i);
        } else {
            self.v = self.v & !(1 << i);
        }
    }

    /// gets stored value
    pub fn get_value(&self) -> u8 {
        self.v
    }

    /// sets the stored value
    pub fn set_value(&mut self, v: u8) {
        self.v = v;
    }
}

impl Reg16 {
    /// constructs a registry from an array of indexes and a default value
    pub fn new(i: [usize; 2], v: u16) -> Reg16 {
        if i[0] == i[1] {
            panic!("indexes cant be the same: {}/{}", i[0], i[1]);
        }

        Reg16 { v, i }
    }

    /// update registry with `MemBuf`
    pub fn get<const S: usize>(&mut self, m: &MemBuf<S>) {
        let v = u16::from_be_bytes(m.get_buf()[self.i[0]..=self.i[1]].try_into().unwrap());
        self.v = v;
    }

    /// gets bit at index i
    pub fn get_bit(&self, i: usize) -> bool {
        if i >= 16 {
            panic!("index out of bounds: {}/{}", i, 16);
        }
        self.v & (1 << i) != 0
    }

    /// sets `MemBuf` value for stored indexes
    pub fn set<const S: usize>(&self, m: &mut MemBuf<S>) {
        let [a, b] = self.v.to_be_bytes();
        m.set(self.i[0], a);
        m.set(self.i[1], b);
    }

    /// sets bit at index i
    pub fn set_bit(&mut self, i: usize, b: bool) {
        if i >= 16 {
            panic!("index out of bounds: {}/{}", i, 16);
        }
        if b {
            self.v = self.v | (1 << i);
        } else {
            self.v = self.v & !(1 << i);
        }
    }

    /// gets the specified byte
    pub fn get_byte(&self, i: usize) -> u8 {
        self.v.to_be_bytes()[i]
    }

    /// sets the specified byte
    pub fn set_byte(&mut self, i: usize, b: u8) {
        let mut v = self.v.to_be_bytes();
        v[i] = b;
        self.v = u16::from_be_bytes(v);
    }

    /// sets the stored value
    pub fn set_value(&mut self, v: u16) {
        self.v = v;
    }

    /// gets stored value
    pub fn get_value(&self) -> u16 {
        self.v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::MemBuf;

    fn test_membuf() -> MemBuf<10> {
        MemBuf::<10>::new([0u8; 10], 4)
    }

    #[test]
    fn test_new() {
        let r = Reg8::new(1, 3);
        let r = Reg16::new([2, 3], 258);
    }

    #[test]
    fn test_get() {
        let m = test_membuf();
        let mut r = Reg8::new(1, 3);
        r.get(&m);
        assert_eq!(r.get_value(), 0);
        let mut r = Reg16::new([1, 2], 5);
        r.get(&m);
        assert_eq!(r.get_value(), 0);
    }

    #[test]
    fn test_get_bit() {
        let r = Reg8::new(1, 3);
        assert!(r.get_bit(1));
        let r = Reg16::new([2, 3], 5);
        assert!(r.get_bit(0));
    }

    #[test]
    fn test_set() {
        let mut m = test_membuf();
        let r = Reg8::new(1, 5);
        r.set(&mut m);
        assert_eq!(m.get_buf()[1], 5);
        let r = Reg16::new([2, 3], 258);
        r.set(&mut m);
        let v = u16::from_be_bytes(m.get_buf()[2..=3].try_into().unwrap());
        assert_eq!(v, 258);
    }

    #[test]
    fn test_set_bit() {
        let mut r = Reg8::new(1, 2);
        r.set_bit(1, false);
        assert!(!r.get_bit(1));
        let mut r = Reg16::new([1, 2], 0);
        r.set_bit(8, true);
        assert!(r.get_bit(8));
    }

    #[test]
    fn test_set_byte() {
        let mut r = Reg16::new([1, 2], 0);
        r.set_byte(0, 5);
        assert_eq!(r.get_byte(0), 5);
    }

    #[test]
    fn test_set_value() {
        let mut r = Reg8::new(1, 2);
        r.set_value(10);
        assert_eq!(r.get_value(), 10);
        let mut r = Reg16::new([1, 2], 0);
        r.set_value(400);
        assert_eq!(r.get_value(), 400);
    }

    #[test]
    fn test_value() {
        let r = Reg8::new(1, 2);
        assert_eq!(r.get_value(), 2);
        let r = Reg16::new([1, 2], 4);
        assert_eq!(r.get_value(), 4);
    }

    #[test]
    fn test_impl() {
        let mut m = test_membuf();
        let mut r = Reg8::new(1, 5);
        r.get(&m);
        r.set(&mut m);
        r.get_bit(7);
        r.set_bit(5, true);
        r.get_value();
        r.set_value(5);

        let mut r = Reg16::new([2, 3], 5);
        r.get(&m);
        r.set(&mut m);
        r.get_bit(10);
        r.set_bit(9, false);
        r.get_value();
        r.set_value(5);
        r.set_byte(1, 5);

        // TODO: implement generic reg struct
    }
}
