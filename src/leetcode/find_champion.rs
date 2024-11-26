pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let graph = edges
        .into_iter()
        .fold(vec![0; n as usize], |mut acc, curr| {
            acc[curr[1] as usize] += 1;
            acc
        });

    let mut res = -1;
    for (node, edges) in graph.into_iter().enumerate() {
        if edges == 0 {
            if res != -1 {
                return -1;
            }
            res = node as i32;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::cell_vec;

    use super::find_champion;

    #[test]
    fn finds_champion_when_it_exists() {
        assert_eq!(0, find_champion(3, cell_vec![(0, 1), (1, 2)]))
    }

    #[test]
    fn negative_one_when_champion_does_not_exist() {
        assert_eq!(-1, find_champion(4, cell_vec![(0, 2), (1, 3), (1, 2)]))
    }
}
