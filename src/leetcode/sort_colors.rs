pub fn sort_colors(numbers: &mut [i32]) {
    let counts = numbers.iter().fold((0, 0, 0), |mut acc, curr| {
        match curr {
            0 => acc.0 += 1,
            1 => acc.1 += 1,
            2 => acc.2 += 1,
            _ => {}
        }
        acc
    });

    let mut index = 0;
    for _ in 0..counts.0 {
        numbers[index] = 0;
        index += 1;
    }
    for _ in 0..counts.1 {
        numbers[index] = 1;
        index += 1;
    }
    for _ in 0..counts.2 {
        numbers[index] = 2;
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::leetcode::sort_colors::sort_colors;

    #[test]
    fn test_sort_colors() {
        let mut numbers = vec![0, 1, 2, 0, 1, 2];
        sort_colors(&mut numbers);

        assert_eq!(&vec![0, 0, 1, 1, 2, 2], &numbers);
    }
}
