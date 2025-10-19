struct CustomStack {
    values: Vec<i32>,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self {
            values: Vec::with_capacity(max_size as usize),
        }
    }

    fn push(&mut self, x: i32) {
        if self.values.len() < self.values.capacity() {
            self.values.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.values.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        for num in self.values.iter_mut().take(k as usize) {
            *num += val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CustomStack;

    #[test]
    fn empty_stack_is_empty() {
        let mut stack = CustomStack::new(5);

        assert_eq!(-1, stack.pop())
    }

    #[test]
    fn pushing_onto_stack_adds_value() {
        let mut stack = CustomStack::new(1);
        stack.push(5);

        assert_eq!(5, stack.pop())
    }

    #[test]
    fn stack_ignores_values_past_capacity() {
        let mut stack = CustomStack::new(1);
        stack.push(5);
        stack.push(4);

        assert_eq!(5, stack.pop())
    }

    #[test]
    fn stack_increments_first_k_values() {
        let mut stack = CustomStack::new(5);
        stack.push(5);
        stack.push(4);
        stack.increment(5, 2);

        assert_eq!(6, stack.pop());
        assert_eq!(7, stack.pop())
    }
}
