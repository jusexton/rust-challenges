fn perimeter(number: u64) -> u64 {
    (Fibonacci::new().take((number + 1) as usize).sum::<usize>() * 4) as u64
}

struct Fibonacci {
    curr: usize,
    next: usize,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::perimeter::{perimeter, Fibonacci};

    #[test_case(1, &[1])]
    #[test_case(2, &[1, 1])]
    #[test_case(3, &[1, 1, 2])]
    #[test_case(5, &[1, 1, 2, 3, 5])]
    fn should_generate_correct_fib_sequence(number: u64, expected: &[usize]) {
        let actual = Fibonacci::new()
            .take(number as usize)
            .collect::<Vec<usize>>();
        assert_eq!(expected.to_vec(), actual)
    }

    #[test_case(5, 80)]
    #[test_case(7, 216)]
    #[test_case(20, 114624)]
    #[test_case(30, 14098308)]
    fn should_return_perimeter_of_n_number_of_squares(n: u64, expected: u64) {
        let actual = perimeter(n);
        assert_eq!(expected, actual);
    }
}
