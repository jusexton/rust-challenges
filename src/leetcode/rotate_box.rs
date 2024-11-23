pub fn rotate_the_box(mut box_: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for row in box_.iter_mut() {
        let mut drop_pos = row.len() - 1;
        for curr_pos in (0..row.len()).rev() {
            match row[curr_pos] {
                '*' => drop_pos = curr_pos.saturating_sub(1),
                '#' => {
                    row.swap(drop_pos, curr_pos);
                    drop_pos = drop_pos.saturating_sub(1);
                }
                _ => {}
            }
        }
    }

    let mut rotated_box = vec![vec!['.'; box_.len()]; box_[0].len()];
    for i in 0..box_.len() {
        (0..box_[0].len()).for_each(|j| {
            rotated_box[j][box_.len() - 1 - i] = box_[i][j];
        });
    }
    rotated_box
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_given_box_and_applies_transformations() {
        assert_eq!(
            vec![vec!['.'], vec!['#'], vec!['#']],
            rotate_the_box(vec![vec!['#', '.', '#']])
        );

        assert_eq!(
            vec![
                vec!['.', '#', '#'],
                vec!['.', '#', '#'],
                vec!['#', '#', '*'],
                vec!['#', '*', '.'],
                vec!['#', '.', '*'],
                vec!['#', '.', '.']
            ],
            rotate_the_box(vec![
                vec!['#', '#', '*', '.', '*', '.'],
                vec!['#', '#', '#', '*', '.', '.'],
                vec!['#', '#', '#', '.', '#', '.']
            ])
        )
    }
}
