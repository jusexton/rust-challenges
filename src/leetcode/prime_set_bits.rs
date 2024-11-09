pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    // Used to map some number to how many primes it represents, in our case its a prime or not.
    // Only possible due to the small problem restrictions.
    let prime_increments = [
        0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0,
    ];
    (left..=right)
        .map(|n| prime_increments[n.count_ones() as usize])
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::prime_set_bits::count_prime_set_bits;

    #[test]
    fn yields_count_in_range_having_prime_number_of_bits() {
        assert_eq!(4, count_prime_set_bits(6, 10));
        assert_eq!(5, count_prime_set_bits(10, 15))
    }
}
