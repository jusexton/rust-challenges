pub struct Counts(usize, usize);

impl Counts {
    fn inc(&mut self, c: char) {
        match c {
            'W' => self.0 += 1,
            _ => self.1 += 1,
        }
    }

    fn reset(&mut self) {
        self.0 = 0;
        self.1 = 0;
    }
}

pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
    let mut counts = Counts(0, 0);

    // first square
    counts.inc(grid[0][0]);
    counts.inc(grid[0][1]);
    counts.inc(grid[1][0]);
    counts.inc(grid[1][1]);
    if counts.0 > 2 || counts.1 > 2 {
        return true;
    }
    counts.reset();

    // second square
    counts.inc(grid[0][1]);
    counts.inc(grid[0][2]);
    counts.inc(grid[1][1]);
    counts.inc(grid[1][2]);
    if counts.0 > 2 || counts.1 > 2 {
        return true;
    }
    counts.reset();

    // third square
    counts.inc(grid[1][0]);
    counts.inc(grid[1][1]);
    counts.inc(grid[2][0]);
    counts.inc(grid[2][1]);
    if counts.0 > 2 || counts.1 > 2 {
        return true;
    }
    counts.reset();

    // forth square
    counts.inc(grid[1][1]);
    counts.inc(grid[1][2]);
    counts.inc(grid[2][1]);
    counts.inc(grid[2][2]);
    if counts.0 > 2 || counts.1 > 2 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_square_with_valid_values() {
        assert!(can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'B'],
        ]));
    }

    #[test]
    fn cant_make_square_with_valid_values() {
        assert!(!can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['W', 'B', 'W'],
            vec!['B', 'W', 'B'],
        ]));
    }
}
