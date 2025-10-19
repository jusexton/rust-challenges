pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
    (start ^ goal).count_ones() as i32
}

#[cfg(test)]
mod tests {
    use super::min_bit_flips;

    #[test]
    fn minimum_number_of_flips() {
        assert_eq!(3, min_bit_flips(10, 7));
        assert_eq!(3, min_bit_flips(3, 4))
    }
}
