fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a = a.iter().map(|number| number.pow(2)).collect::<Vec<i64>>();
    let mut b = b.clone();
    a.sort();
    b.sort();
    return a == b;
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::same::comp;

    #[test_case(&[], &[], true)]
    #[test_case(&[1, 2, 3], &[1, 4, 9], true)]
    #[test_case(&[1, 2, 3], &[1, 4, 9, 10], false)]
    fn should_return_whether_the_two_given_arrays_are_same(a: &[i64], b: &[i64], expected: bool) {
        let actual = comp(a.to_vec(), b.to_vec());
        assert_eq!(expected, actual);
    }
}
