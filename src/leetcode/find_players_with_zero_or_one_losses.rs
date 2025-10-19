use std::collections::BTreeMap;

fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut players = BTreeMap::new();
    for m in matches {
        if let [winner, loser] = m.as_slice() {
            players.entry(*winner).or_insert(0);
            *players.entry(*loser).or_insert(0) += 1;
        }
    }

    players
        .iter()
        .fold(vec![vec![], vec![]], |mut acc, (player, losses)| {
            match losses {
                0 => acc[0].push(*player),
                1 => acc[1].push(*player),
                _ => {}
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use crate::leetcode::find_players_with_zero_or_one_losses::find_winners;

    #[test]
    fn test_find_winners() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        let winners = find_winners(matches);

        let expected = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];
        assert_eq!(winners, expected);
    }
}
