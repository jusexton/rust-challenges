fn multiplication_table(length: usize) -> Vec<Vec<usize>> {
    let mut table: Vec<Vec<usize>> = Vec::new();
    for row in 1..length + 1 {
        let row = (1..length + 1).map(|number| number * row).collect();
        table.push(row);
    }

    return table;
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::multiplication::multiplication_table;

    #[test_case(0, &[])]
    #[test_case(1, &[&[1]])]
    #[test_case(2, &[&[1, 2], &[2, 4]])]
    #[test_case(3, &[&[1, 2, 3], &[2, 4, 6], &[3, 6, 9]])]
    fn should_return_multiplication_table_of_specified_length(length: usize, expected: &[&[usize]]) {
        let actual = multiplication_table(length);
        let expected = expected.iter().map(|row| row.to_vec()).collect::<Vec<Vec<usize>>>();
        assert_eq!(expected, actual);
    }
}