pub fn remove_kdigits(number: String, k: i32) -> String {
    let mut k = k as usize;
    let mut stack = String::with_capacity(number.len() - k);
    for digit in number.chars() {
        while k > 0 && !stack.is_empty() && digit < stack.chars().last().unwrap() {
            // Remove digits larger than the current one.
            stack.pop();
            k -= 1;
        }
        if stack.is_empty() && digit == '0' {
            // Skip leading zeros.
            continue;
        }
        stack.push(digit);
    }

    // Remove any remaining right digits if not all digits were removed in the main loop.
    for _ in 0..k {
        stack.pop();
    }

    match stack.is_empty() {
        true => "0".to_string(),
        false => stack,
    }
}

#[cfg(test)]
mod tests {
    use super::remove_kdigits;

    #[test]
    fn removes_k_digits_to_achieve_smallest_number_possible() {
        assert_eq!("1219".to_string(), remove_kdigits("1432219".to_string(), 3))
    }

    #[test]
    fn digits_with_trailing_zeros() {
        assert_eq!("200".to_string(), remove_kdigits("10200".to_string(), 1))
    }

    #[test]
    fn remove_all_digits() {
        assert_eq!("0".to_string(), remove_kdigits("10".to_string(), 2))
    }
}
