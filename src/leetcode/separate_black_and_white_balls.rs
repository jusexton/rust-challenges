pub fn minimum_steps(s: String) -> i64 {
    let (mut swap, mut black) = (0, 0);
    for ch in s.chars() {
        if ch == '0' {
            swap += black;
        } else {
            black += 1;
        }
    }

    swap
}

#[cfg(test)]
mod tests {
    use super::minimum_steps;

    #[test]
    fn no_steps_when_already_sorted() {
        assert_eq!(0, minimum_steps("0011".to_string()))
    }

    #[test]
    fn correct_number_of_steps() {
        assert_eq!(2, minimum_steps("100".to_string()))
    }
}
