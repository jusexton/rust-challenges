pub fn binary_gap(mut n: i32) -> i32 {
    let mut largest_gap = i32::MIN;
    while n > 0 {
        if n & 1 == 0 {
            n >>= 1;
            continue;
        }

        n >>= 1;
        let mut gap = 0;
        while n > 0 && n & 1 == 0 {
            n >>= 1;
            gap += 1;
        }

        let abs_gap = if n & 1 == 1 { gap + 1 } else { 0 };
        largest_gap = largest_gap.max(abs_gap)
    }

    if largest_gap == i32::MIN {
        0
    } else {
        largest_gap
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::binary_gap::binary_gap;

    #[test_case(8, 0)]
    #[test_case(22, 2)]
    #[test_case(5, 2)]
    #[test_case(6, 1)]
    fn test_binary_gap(input: i32, expected: i32) {
        assert_eq!(expected, binary_gap(input))
    }
}
