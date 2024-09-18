fn largest_number(numbers: Vec<i32>) -> String {
    let mut numbers: Vec<String> = numbers.iter().map(|item| item.to_string()).collect();
    numbers.sort_unstable_by(|a, b| {
        let ab = format!("{}{}", a, b).as_str().parse::<i128>().unwrap();
        let ba = format!("{}{}", b, a).as_str().parse::<i128>().unwrap();
        ba.cmp(&ab)
    });
    if numbers[0] == "0" {
        "0".to_string()
    } else {
        numbers.join("")
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::largest_number::largest_number;

    #[test_case(&[0,0], "0")]
    #[test_case(&[1,2,3], "321")]
    #[test_case(&[1,45,9], "9451")]
    #[test_case(&[3,30,34,5,9], "9534330")]
    fn should_return_largest_possible_number(numbers: &[i32], expected: &str) {
        let result = largest_number(numbers.to_vec());
        let expected = String::from(expected);
        assert_eq!(result, expected);
    }
}
