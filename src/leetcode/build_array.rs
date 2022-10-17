fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut index = 1;
    let mut collected = 0;

    while index <= n && collected < target.len() {
        result.push(String::from("Push"));
        if target[collected] == index {
            collected += 1
        } else {
            result.push(String::from("Pop"));
        }

        index += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::build_array::build_array;

    #[test_case(&[1], 3, &["Push"])]
    #[test_case(&[1, 2, 3], 3, &["Push", "Push", "Push"])]
    #[test_case(&[1, 3], 3, &["Push", "Push", "Pop", "Push"])]
    fn should_return_how_to_reach_given_target(target: &[i32], n: i32, expected: &[&str]) {
        let expected = to_string_vec(expected);
        let actual = build_array(target.to_vec(), n);
        assert_eq!(expected, actual)
    }

    fn to_string_vec(slices: &[&str]) -> Vec<String> {
        slices.iter().map(|it| String::from(*it)).collect()
    }
}
