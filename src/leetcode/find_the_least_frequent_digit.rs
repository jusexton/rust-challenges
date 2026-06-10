pub fn get_least_frequent_digit(mut n: i32) -> i32 {
    let mut freq = [0; 10];
    while n > 0 {
        let digit = n % 10;
        freq[digit as usize] += 1;
        n /= 10;
    }

    let (mut min_freq, mut result) = (i32::MAX, 0i32);
    for digit in 0..10 {
        if freq[digit] > 0 && freq[digit] < min_freq {
            min_freq = freq[digit];
            result = digit as i32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_least_frequent_digit() {
        assert_eq!(1, get_least_frequent_digit(15544));
        assert_eq!(4, get_least_frequent_digit(5546));
        assert_eq!(1, get_least_frequent_digit(1553322));
        assert_eq!(2, get_least_frequent_digit(723344511));
    }
}
