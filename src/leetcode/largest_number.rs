fn largest_number(nums: Vec<i32>) -> String {
    let mut nums = nums
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>();

    nums.sort_by(|a, b| {
        let ab = format!("{}{}", a, b);
        let ba = format!("{}{}", b, a);
        ba.cmp(&ab)
    });

    let result = nums.into_iter().collect::<String>();
    if result.starts_with('0') {
        String::from("0")
    } else {
        result
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
