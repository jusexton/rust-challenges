use std::collections::HashMap;

pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
    let seats = reserved_seats
        .into_iter()
        .fold(HashMap::new(), |mut acc, curr| {
            let (row, column) = (curr[0], curr[1]);
            *acc.entry(row).or_insert(0) |= 1 << column;
            acc
        });

    let max = seats.values().fold(0i32, |mut acc, curr| {
        let mut count = 0;
        if (curr & 60) == 0 {
            count += 1;
        }
        if (curr & 960) == 0 {
            count += 1;
        }
        if (curr & 240) == 0 && count == 0 {
            count = 1;
        }

        acc += count;
        acc
    });

    let empty_rows = (n as usize - seats.len()) as i32;
    max + 2 * empty_rows
}
#[cfg(test)]
mod tests {
    use super::max_number_of_families;

    #[test]
    fn calculates_max_number_of_families() {
        assert_eq!(
            2,
            max_number_of_families(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]])
        )
    }
}
