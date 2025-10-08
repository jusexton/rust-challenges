use std::cmp::Ordering;

fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let potion_count = potions.len();
    potions.sort_unstable();
    spells
        .into_iter()
        .map(|spell| {
            let idx = potions
                .binary_search_by(|&potion_power| {
                    match (spell as i64 * potion_power as i64).cmp(&success) {
                        Ordering::Less => Ordering::Less,
                        Ordering::Greater => Ordering::Greater,
                        Ordering::Equal => Ordering::Greater,
                    }
                })
                .unwrap_err();

            (potion_count - idx) as i32
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::spells_and_potions::successful_pairs;

    #[test]
    fn finds_correct_pair_count() {
        assert_eq!(
            vec![4, 0, 3],
            successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7)
        );

        assert_eq!(
            vec![2, 0, 2],
            successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16)
        );
    }
}
