pub fn min_swaps(s: String) -> i32 {
    let open_count = s.chars().fold(0, |mut acc, curr| {
        if curr == '[' {
            acc += 1;
        } else if acc > 0 {
            acc -= 1;
        }
        acc
    });
    (open_count + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::min_swaps;

    #[test]
    fn returns_correct_number_of_swaps() {
        assert_eq!(1, min_swaps("][][".to_string()));
        assert_eq!(2, min_swaps("]]][[[".to_string()))
    }

    #[test]
    fn no_swaps_required_returns_zero() {
        assert_eq!(0, min_swaps("[][]".to_string()))
    }
}
