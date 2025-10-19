pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let mut positions = vec![];
    let mut start = 0;
    for idx in 0..=s.len() {
        let current = s.chars().nth(idx).unwrap_or(' ');
        let marked = s.chars().nth(start).unwrap();
        if idx == s.len() || current != marked {
            if idx - start >= 3 {
                positions.push(vec![start as i32, (idx - 1) as i32]);
            }
            start = idx;
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::large_group_positions;

    #[test]
    fn exact_group() {
        assert_eq!(vec![vec![0, 2]], large_group_positions("aaa".to_string()))
    }

    #[test]
    fn single_group() {
        assert_eq!(
            vec![vec![3, 6]],
            large_group_positions("abbxxxxzzy".to_string())
        )
    }

    #[test]
    fn many_groups() {
        assert_eq!(
            vec![vec![3, 5], vec![6, 9], vec![12, 14]],
            large_group_positions("abcdddeeeeaabbbcd".to_string())
        )
    }
}
