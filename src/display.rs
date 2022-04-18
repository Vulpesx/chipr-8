
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::MemBuf;

    #[test]
    fn test_impl() {
        let m = MemBuf::default();
        let d = Display::new(64, 32);
    }
}
