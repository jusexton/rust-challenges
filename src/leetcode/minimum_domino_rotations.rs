pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    let n = tops.len();

    let mut top_freqs = [0; 7];
    let mut bottom_freqs = [0; 7];
    let mut doubles = [0; 7];
    for (top, bottom) in tops.into_iter().zip(bottoms) {
        top_freqs[top as usize] += 1;
        bottom_freqs[bottom as usize] += 1;
        if top == bottom {
            doubles[top as usize] += 1;
        }
    }

    for i in 1..7 {
        if top_freqs[i] + bottom_freqs[i] - doubles[i] == n {
            return (n - top_freqs[i].max(bottom_freqs[i])) as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::min_domino_rotations;

    #[test]
    fn minimum_domino_rotations() {
        assert_eq!(
            2,
            min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2])
        );
    }

    #[test]
    fn no_rotation_combination() {
        assert_eq!(
            -1,
            min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4])
        );
    }
}
