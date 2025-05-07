pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut domino_counts = vec![0; 256];
    for domino in dominoes {
        let (a, b) = (domino[0], domino[1]);
        let (smaller, larger) = match b > a {
            true => (a, b),
            false => (b, a),
        };

        let hash = (smaller << 4) | larger;
        domino_counts[hash as usize] += 1;
    }

    let mut total_pairs = 0;
    for count in domino_counts {
        if count > 1 {
            total_pairs += count * (count - 1) / 2;
        }
    }

    total_pairs
}

#[cfg(test)]
mod tests {
    use super::num_equiv_domino_pairs;

    #[test]
    fn finds_equivalent_domino_pairs() {
        assert_eq!(
            1,
            num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]])
        );

        assert_eq!(
            3,
            num_equiv_domino_pairs(vec![
                vec![1, 2],
                vec![1, 2],
                vec![1, 1],
                vec![1, 2],
                vec![2, 2]
            ])
        );
    }
}
