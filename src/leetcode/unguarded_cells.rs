pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let mut grid = vec![vec![0; n as usize]; m as usize];
    for guard in guards.iter() {
        grid[guard[0] as usize][guard[1] as usize] = 2;
    }
    for wall in walls.iter() {
        grid[wall[0] as usize][wall[1] as usize] = 2;
    }

    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for guard in guards.iter() {
        let (gx, gy) = (guard[0], guard[1]);
        for (dx, dy) in directions.iter() {
            let (mut x, mut y) = (gx, gy);
            while x + dx >= 0
                && x + dx < m
                && y + dy >= 0
                && y + dy < n
                && grid[(x + dx) as usize][(y + dy) as usize] != 2
            {
                x += dx;
                y += dy;
                grid[x as usize][y as usize] = 1;
            }
        }
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell == 0)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use crate::cell_vec;

    use super::*;

    #[test]
    fn counts_number_of_unguarded_cells() {
        assert_eq!(
            7,
            count_unguarded(
                4,
                6,
                cell_vec![(0, 0), (1, 1), (2, 3)],
                cell_vec![(0, 1), (2, 2), (1, 4)]
            )
        )
    }
}
