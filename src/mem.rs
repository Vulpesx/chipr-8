
/// A buffer struct with the ability to imitate chip-8 memory but better.
pub struct MemBuf<const SIZE: usize = 512> {
    buf: [u8; SIZE],
    start: usize,
}

impl<const SIZE: usize> MemBuf<SIZE> {
    /// construct a `MemBuf` from an array and a start index
    pub fn new<const S: usize>(buf: [u8; S], start: usize) -> MemBuf<S> {
        MemBuf::<S> { buf, start }
    }

    /// set a byte in the buffer
    pub fn set(&mut self, i: usize, b: u8) {
        self.buf[i] = b;
    }

    /// set a bit in the buffer
    pub fn set_bit(&mut self, i: usize, s: bool) {
        if i > self.buf.len() * 8 {
            panic!("index out of bounds: {}/{}", i, self.buf.len() * 8);
        }
        let b = (i - (i % 8)) / 8;
        let i = i % 8;

        if s {
            self.buf[b] = self.buf[b] | (1 << i);
        } else {
            self.buf[b] = self.buf[b] & !(1 << i);
        }
    }

    /// get a byte in the buffer
    pub fn get(&self, i: usize) -> u8 {
        self.buf[i]
    }

    /// get a bit in the buffer
    pub fn get_bit(&self, i: usize) -> bool {
        if i > self.buf.len() * 8 {
            panic!("index out of bounds: {}/{}", i, self.buf.len() * 8);
        }
        let b = (i - (i % 8)) / 8;
        let i = i % 8;

        return (self.buf[b] & (1 << i)) != 0;
    }

    /// get the byte at the start location
    pub fn start(&self) -> u8 {
        self.buf[self.start]
    }

    /// get the bit at the start location
    pub fn start_bit(&self) -> bool {
        self.get_bit(self.start * 8)
    }

    /// get the buffer in the form of a slice
    pub fn get_buf(&self) -> &[u8] {
        &self.buf
    }

    /// get the starting index
    pub fn get_start(&self) -> usize {
        self.start
    }
}

impl Default for MemBuf {

    fn default() -> Self {
        MemBuf::<512>::new([0u8; 512], 0x200)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = MemBuf::<3>::new([1; 3], 2);
        assert_eq!(m.get_buf(), [1, 1, 1]);
        assert_eq!(m.get_start(), 2);
    }

    #[test]
    fn test_get() {
        let m = MemBuf::<5>::new([6u8; 5], 0);
        assert_eq!(m.get(3), 6);
    }

    #[test]
    fn test_get_bit() {
        let m = MemBuf::<5>::new([1; 5], 0);
        assert!(m.get_bit(8));
    }

    #[test]
    fn test_set() {
        let mut m = MemBuf::default();
        m.set(5, 12);
        assert_eq!(m.get(5), 12);
    }

    #[test]
    fn test_set_bit() {
        let mut m = MemBuf::default();
        m.set_bit(20, true);
        assert!(m.get_bit(20));
    }

    #[test]
    fn test_impl() {
        let m = MemBuf::<512>::default();
        let mut m = MemBuf::<512>::new([5u8; 512], 0x600 / 8);
        m.set(12, 8);
        m.set_bit(3000, false);
        m.get(44);
        m.get_bit(20);
        m.start();
        m.start_bit();
    }
}
