use crate::{reg::Reg8, mem::MemBuf};

pub struct Timer<'a> {
    r: &'a mut Reg8,
    prev: f32,
    t: f32,
    h: usize,
    c: fn(u8),
}

impl<'a> Timer<'a> {
    pub fn new(h: usize, r: &mut Reg8, c: fn(u8)) -> Timer {
        Timer {
            h,
            r,
            c,
            prev: 0f32,
            t: 0f32,
        }
    }

    pub fn update<const S: usize>(&mut self, m: &mut MemBuf<S>) -> bool {
        self.r.get(m);
        if self.r.value() == 0 { return false; }
        if self.prev >= 1.0 / self.h as f32 {
            self.prev = 0f32;
            self.r.value() - 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{mem::MemBuf, reg::Reg8};
    use super::*;

    #[test]
    fn test_update() {
        let mut m = MemBuf::new([0; 5], 0);
        let mut r = Reg8::new(0, 0);
        r.get(&m); // good practice
        r.set_bit(2, true); // = 4
        r.set(&mut m);
        let mut t = Timer::new(60, &mut r, |v| {println!("buzz: {}", v);});
        while t.update(&mut m) {}
        assert_eq!(t.get_value(), 0);
    }

    #[test]
    fn test_impl() {
        let mut m = MemBuf::new([0;5], 0);
        let mut r = Reg8::new(0, 0);
        let mut t = Timer::new(60, &mut r, |v| {});
        t.update(&mut m);
        t.get_value();
    }
}
