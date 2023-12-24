use std::cell::RefCell;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

struct Bitset {
    size: i32,
    ones: Rc<RefCell<HashSet<i32>>>,
    zeros: Rc<RefCell<HashSet<i32>>>,
}

impl Bitset {
    fn new(size: i32) -> Self {
        Self {
            size,
            ones: Rc::new(RefCell::new(HashSet::new())),
            zeros: Rc::new(RefCell::new(HashSet::from_iter(0..size))),
        }
    }

    fn fix(&mut self, idx: i32) {
        self.ones.borrow_mut().insert(idx);
        self.zeros.borrow_mut().remove(&idx);
    }

    fn unfix(&mut self, idx: i32) {
        self.zeros.borrow_mut().insert(idx);
        self.ones.borrow_mut().remove(&idx);
    }

    fn flip(&mut self) {
        let temp = self.ones.clone();
        self.ones = self.zeros.clone();
        self.zeros = temp;
    }

    fn all(&self) -> bool {
        self.ones.borrow().len() == self.size as usize
    }

    fn one(&self) -> bool {
        self.ones.borrow().len() > 0
    }

    fn count(&self) -> i32 {
        self.ones.borrow().len() as i32
    }

    fn to_string(&self) -> String {
        let ones = self.ones.borrow();
        (0..self.size)
            .map(|idx| match ones.contains(&idx) {
                true => '1',
                false => '0',
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::bitset::Bitset;

    #[test]
    fn test_initialization() {
        let mut bitset = Bitset::new(3);
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
