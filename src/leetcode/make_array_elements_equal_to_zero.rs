pub fn count_valid_selections(numbers: Vec<i32>) -> i32 {
    fn valid_simulation(pos: usize, mut numbers: Vec<i32>, mut direction: i32) -> bool {
        let mut pos = pos as i32;
        while pos >= 0 && pos < numbers.len() as i32 {
            if numbers[pos as usize] > 0 {
                direction *= -1;
                numbers[pos as usize] -= 1;
            }
            pos += direction;
        }

        numbers.into_iter().all(|n| n == 0)
    }

    let starting_positions = (0..numbers.len()).filter(|&i| numbers[i] == 0);
    starting_positions.fold(0, |mut acc, curr| {
        valid_simulation(curr, numbers.clone(), 1).then(|| acc += 1);
        valid_simulation(curr, numbers.clone(), -1).then(|| acc += 1);
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::leetcode::make_array_elements_equal_to_zero::count_valid_selections;

    #[test]
    fn test_count_valid_selections() {
        assert_eq!(2, count_valid_selections(vec![1, 0, 2, 0, 3]));
    }
}
