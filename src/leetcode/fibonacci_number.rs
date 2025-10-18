use std::collections::HashMap;

/// https://leetcode.com/problems/fibonacci-number
fn fibonacci(target: u32) -> u32 {
    fibonacci_memoized(target, &mut HashMap::new())
}

fn fibonacci_memoized(target: u32, cache: &mut HashMap<u32, u32>) -> u32 {
    match target {
        0 => 0,
        1 => 1,
        n => cache.get(&n).copied().unwrap_or_else(|| {
            let result = fibonacci_memoized(n - 1, cache) + fibonacci_memoized(n - 2, cache);
            cache.insert(n, result);
            result
        }),
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::fibonacci_number::fibonacci;

    #[test_case(0, 0)]
    #[test_case(1, 1)]
    #[test_case(2, 1)]
    #[test_case(3, 2)]
    #[test_case(4, 3)]
    #[test_case(25, 75_025)]
    fn fibonacci_tests(input: u32, expected: u32) {
        assert_eq!(expected, fibonacci(input))
    }
}
