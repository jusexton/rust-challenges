use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::mem;

struct Bitset {
    size: i32,
    ones: HashSet<i32>,
    zeros: HashSet<i32>,
}

impl Bitset {
    fn new(size: i32) -> Self {
        Self {
            size,
            ones: HashSet::new(),
            zeros: HashSet::from_iter(0..size),
        }
    }

    fn fix(&mut self, idx: i32) {
        self.ones.insert(idx);
        self.zeros.remove(&idx);
    }

    fn unfix(&mut self, idx: i32) {
        self.zeros.insert(idx);
        self.ones.remove(&idx);
    }

    fn flip(&mut self) {
        mem::swap(&mut self.ones, &mut self.zeros);
    }

    fn all(&self) -> bool {
        self.ones.len() == self.size as usize
    }

    fn one(&self) -> bool {
        !self.ones.is_empty()
    }

    fn count(&self) -> i32 {
        self.ones.len() as i32
    }
}

impl Display for Bitset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result: String = (0..self.size)
            .map(|idx| match self.ones.contains(&idx) {
                true => '1',
                false => '0',
            })
            .collect();
        write!(f, "{result}")
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::bitset::Bitset;

    #[test]
    fn test_initialization() {
        let bitset = Bitset::new(3);
        assert_eq!(bitset.to_string(), "000")
    }

    #[test]
    fn test_fix() {
        let mut bitset = Bitset::new(3);
        bitset.fix(0);

        assert_eq!(bitset.to_string(), "100")
    }

    #[test]
    fn test_unfix() {
        let mut bitset = Bitset::new(3);
        bitset.fix(0);
        bitset.unfix(0);

        assert_eq!(bitset.to_string(), "000")
    }

    #[test]
    fn test_flip() {
        let mut bitset = Bitset::new(3);
        bitset.flip();

        assert_eq!(bitset.to_string(), "111")
    }

    #[test]
    fn test_all() {
        let mut bitset = Bitset::new(3);
        assert!(!bitset.all());

        bitset.flip();
        assert!(bitset.all());
    }

    #[test]
    fn test_one() {
        let mut bitset = Bitset::new(3);
        assert!(!bitset.one());

        bitset.fix(0);
        assert!(bitset.one());
    }

    #[test]
    fn test_count() {
        let mut bitset = Bitset::new(3);
        assert_eq!(bitset.count(), 0);

        bitset.flip();
        assert_eq!(bitset.count(), 3);
    }
}
