use std::collections::HashMap;

use rand::{rngs::ThreadRng, seq::IndexedRandom};

struct RandomizedSet {
    values: Vec<i32>,
    position: HashMap<i32, usize>,
    rng: ThreadRng,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            values: Vec::default(),
            position: HashMap::default(),
            rng: rand::rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.position.contains_key(&val) {
            return false;
        }
        self.position.insert(val, self.values.len());
        self.values.push(val);
        true
    }

    fn remove(&mut self, value: i32) -> bool {
        if let Some(index) = self.position.remove(&value) {
            self.values.swap_remove(index);
            if index < self.values.len() {
                self.position.insert(self.values[index], index);
            }
            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        *self.values.choose(&mut self.rng).unwrap()
    }
}
