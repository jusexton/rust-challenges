// ATTEMPT: https://leetcode.com/problems/grid-illumination
// Requires optimization

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),   // right
    (-1, 0),  // left
    (0, -1),  // up
    (0, 1),   // down
    (1, 1),   // right down
    (-1, -1), // left up
    (-1, 1),  // left down
    (1, -1),  // right up
];

enum Position {
    InBounds(i32, i32),
    OutOfBounds,
}

fn find_next_position((x, y): (i32, i32), dir: (i32, i32), n: i32) -> Position {
    let (new_x, new_y) = (x + dir.0, y + dir.1);
    match new_x >= 0 && new_x < n && new_y >= 0 && new_y < n {
        true => Position::InBounds(new_x, new_y),
        false => Position::OutOfBounds,
    }
}

fn adjacent_points(point: (i32, i32), n: i32) -> Vec<(i32, i32)> {
    DIRECTIONS
        .iter()
        .filter_map(|&dir| match find_next_position(point, dir, n) {
            Position::InBounds(x, y) => Some((x, y)),
            Position::OutOfBounds => None,
        })
        .collect()
}

fn is_inline_lit(grid: &[Vec<i32>], current: (i32, i32), n: i32) -> bool {
    for direction in DIRECTIONS {
        let mut point = current;
        while let Position::InBounds(x, y) = find_next_position(point, direction, n) {
            if grid[x as usize][y as usize] == 1 {
                return true;
            }
            point = (x, y)
        }
    }
    false
}

pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut grid = vec![vec![0; n as usize]; n as usize];
    for lamp in lamps {
        let (x, y) = (lamp[0] as usize, lamp[1] as usize);
        grid[x][y] = 1;
    }

    let mut results = vec![];
    for query in queries {
        let current = (query[0], query[1]);
        let current_is_lit = grid[current.0 as usize][current.1 as usize] == 1;
        let result = if current_is_lit || is_inline_lit(&grid, current, n) {
            grid[current.0 as usize][current.1 as usize] = 0;
            for (x, y) in adjacent_points(current, n) {
                grid[x as usize][y as usize] = 0;
            }
            1
        } else {
            0
        };
        results.push(result);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queries_illuminated_lamps() {
        assert_eq!(
            vec![1, 0],
            grid_illumination(
                1,
                vec![vec![0, 0], vec![0, 0]],
                vec![vec![0, 0], vec![0, 0]]
            )
        );

        assert_eq!(
            vec![1, 1],
            grid_illumination(
                5,
                vec![vec![0, 0], vec![4, 4]],
                vec![vec![1, 1], vec![1, 1]]
            )
        );

        assert_eq!(
            vec![1, 1, 0],
            grid_illumination(
                5,
                vec![vec![0, 0], vec![0, 4]],
                vec![vec![0, 4], vec![0, 1], vec![1, 4]]
            )
        );
    }
}
