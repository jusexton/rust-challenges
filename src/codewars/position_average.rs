fn position_average(s: &str) -> f64 {
    let values = s.split(", ").collect::<Vec<&str>>();
    let mut equal_count = 0;
    let mut count = 0;
    for (index, value) in values[..values.len() - 1].iter().enumerate() {
        for v in values[index + 1..].iter() {
            equal_count += value.chars().zip(v.chars()).filter(|(a, b)| a == b).count();
            count += value.len()
        }
    }

    (equal_count as f64 / count as f64) * 100.0
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::position_average::position_average;

    // TODO: Additional test cases would be great but cant be bothered to add the float_eq macro atm.
    #[test_case("123, 123", 100.0)]
    fn test_position_average(input: &str, expected: f64) {
        let average = position_average(input);
        assert_eq!(average, expected)
    }
}
