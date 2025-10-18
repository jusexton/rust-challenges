pub fn maximum_odd_binary_number(s: String) -> String {
    let one_count = s.chars().filter(|c| *c == '1').count();
    let ones: String = "1".repeat(one_count - 1);
    let zeroes: String = "0".repeat(s.len() - one_count);
    format!("{ones}{zeroes}1")
}

#[cfg(test)]
mod tests {
    use super::maximum_odd_binary_number;

    #[test]
    fn test_maximum_odd_binary_is_produced() {
        let actual = maximum_odd_binary_number("101101".to_string());
        assert_eq!("111001", actual)
    }
}
