pub fn remove_element(numbers: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    for i in 0..numbers.len() {
        if numbers[i] != val {
            numbers[index] = numbers[i];
            index += 1;
        }
    }
    println!("{numbers:?}");

    index as i32
}

#[cfg(test)]
mod tests {
    use super::remove_element;

    #[test]
    fn removes_specified_elements_from_array() {
        assert_eq!(2, remove_element(&mut vec![3, 2, 2, 3], 3))
    }
}
