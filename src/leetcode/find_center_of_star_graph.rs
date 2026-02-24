pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    match (&edges[0][..], &edges[1][..]) {
        ([a, b], [c, d]) if a == c || a == d => *a,
        ([_, b], _) => *b,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::find_center_of_star_graph::find_center;

    #[test]
    fn test_find_center() {
        assert_eq!(2, find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]));
        assert_eq!(
            1,
            find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]])
        );
    }
}
