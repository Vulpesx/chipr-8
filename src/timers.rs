use std::time::Instant;
use crate::{reg::Reg8, mem::MemBuf};

pub struct Timer<'a> {
    r: &'a mut Reg8,
    prev: Instant,
    h: usize,
    c: fn(f32, u8),
}

impl<'a> Timer<'a> {
    pub fn new(h: usize, r: &mut Reg8, c: fn(f32, u8)) -> Timer {
        Timer {
            h,
            r,
            c,
            prev: Instant::now(),
        }
    }

    pub fn update<const S: usize>(&mut self, m: &mut MemBuf<S>) -> bool {
        self.r.get(m);
        let v = self.r.get_value();
        if v == 0 { return false; }

        let since = self.prev.elapsed().as_secs_f32();
        if since >= 1.0 / self.h as f32 {
            (self.c)(since, v);
            self.prev = Instant::now();
            self.r.set_value(v - 1);
            self.r.set(m);
        }
        true
    }

    pub fn get_value(&self) -> u8 {
        self.r.get_value()
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
        r.set_value(4);
        r.set(&mut m);
        let mut t = Timer::new(60, &mut r, |s, v| {println!("buzz: {}:{}", v, s);});
        assert_eq!(t.get_value(), 4);
        while t.update(&mut m) {}
        assert_eq!(t.get_value(), 0);
    }

    #[test]
    fn test_impl() {
        let mut m = MemBuf::new([0;5], 0);
        let mut r = Reg8::new(0, 0);
        let mut t = Timer::new(60, &mut r, |_, v| {});
        t.update(&mut m);
        t.get_value();
    }
}
