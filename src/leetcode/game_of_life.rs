const DIRECTIONS: [(i32, i32); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (-1, -1),
    (-1, 1),
    (1, 0),
    (1, -1),
    (1, 1),
];

fn count_alive_neighbors(row: usize, col: usize, board: &[Vec<i32>]) -> i32 {
    let mut alive = 0;
    for (dy, dx) in DIRECTIONS.iter() {
        let (updated_row, updated_col) = (row as i32 + dy, col as i32 + dx);
        if updated_row >= 0
            && updated_row < board.len() as i32
            && updated_col >= 0
            && updated_col < board[0].len() as i32
        {
            alive += board[updated_row as usize][updated_col as usize];
        }
    }
    alive
}

pub fn game_of_life(board: &mut [Vec<i32>]) {
    let mut updates = vec![];
    let (m, n) = (board.len(), board[0].len());
    (0..m).for_each(|i| {
        for j in 0..n {
            let cell = board[i][j];
            let alive_neighbors = count_alive_neighbors(i, j, board);
            match cell {
                1 if alive_neighbors < 2 => updates.push((i, j, 0)),
                1 if alive_neighbors > 3 => updates.push((i, j, 0)),
                0 if alive_neighbors == 3 => updates.push((i, j, 1)),
                _ => {}
            }
        }
    });

    for (i, j, val) in updates {
        board[i][j] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_next_state_in_game_of_life() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        game_of_life(&mut board);
        assert_eq!(
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]],
            board
        )
    }
}
