pub fn longest_string(x: i32, y: i32, z: i32) -> i32 {
    match x == y {
        true => 4 * x + 2 * z,
        false => {
            let min = x.min(y);
            2 * min + (min + 1) * 2 + z * 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_length_of_longest_possible_string() {
        assert_eq!(12, longest_string(2, 5, 1))
    }
}
