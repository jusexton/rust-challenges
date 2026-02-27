// Credit to: https://leetcode.com/problems/minimum-operations-to-equalize-binary-string/solutions/7138938/python-math-by-awice-7jq6/?envType=daily-question&envId=2026-02-27
// Simply converted original solution to Rust.
pub fn min_operations(s: String, k: i32) -> i32 {
    let k = k as usize;
    let bit_count = s.len();
    let zero_count = s.bytes().filter(|&b| b == b'0').count();

    if bit_count == k {
        return match zero_count {
            0 => 0,
            z if z == bit_count => 1,
            _ => -1,
        };
    }

    let mut ops = usize::MAX;

    if zero_count % 2 == 0 {
        let mut moves = zero_count
            .div_ceil(k)
            .max(zero_count.div_ceil(bit_count - k));
        moves += moves & 1;
        ops = ops.min(moves);
    }

    if zero_count % 2 == k % 2 {
        let mut moves = zero_count
            .div_ceil(k)
            .max((bit_count - zero_count).div_ceil(bit_count - k));
        moves += (moves & 1 == 0) as usize;
        ops = ops.min(moves);
    }

    if ops == usize::MAX { -1 } else { ops as i32 }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::minimum_operations_to_equalize_binary_string::min_operations;

    #[test_case("110", 1, 1)]
    #[test_case("0101", 3, 2)]
    #[test_case("101", 2, -1)]
    #[test_case("00", 1, 2)]
    fn test_min_operations(s: &str, k: i32, expected: i32) {
        assert_eq!(expected, min_operations(s.to_string(), k))
    }
}
