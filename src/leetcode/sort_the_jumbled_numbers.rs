use std::collections::HashMap;

pub fn sort_jumbled(mapping: Vec<i32>, mut numbers: Vec<i32>) -> Vec<i32> {
    let mut m = HashMap::with_capacity(numbers.len());
    for (index, number) in numbers.iter().enumerate() {
        let mapped = map_number(*number, &mapping);
        m.insert(*number, (mapped, index));
    }

    numbers.sort_by_key(|number| m.get(number));
    numbers
}

fn map_number(mut number: i32, mapping: &[i32]) -> i32 {
    if number == 0 {
        return mapping[0];
    }

    let mut mapped = 0;
    let mut place = 0;
    while number > 0 {
        mapped += mapping[number as usize % 10] * 10_i32.pow(place);
        number /= 10;
        place += 1;
    }
    mapped
}

#[cfg(test)]
mod tests {
    use super::sort_jumbled;

    #[test]
    fn jumped_numbers_are_sorted() {
        let mapping = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let actual = sort_jumbled(mapping, numbers);
        assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], actual)
    }
}
