fn triangle_type(numbers: Vec<i32>) -> String {
    if numbers[0] + numbers[1] <= numbers[2]
        || numbers[0] + numbers[2] <= numbers[1]
        || numbers[1] + numbers[2] <= numbers[0]
    {
        return "none".to_string();
    }

    if numbers[0] == numbers[1] && numbers[0] == numbers[2] {
        return "equilateral".to_string();
    }

    if (numbers[0] == numbers[1] && numbers[0] != numbers[2])
        || (numbers[1] == numbers[2] && numbers[1] != numbers[0])
        || (numbers[0] == numbers[2] && numbers[0] != numbers[1])
    {
        return "isosceles".to_string();
    }

    if numbers[0] != numbers[1] && numbers[0] != numbers[2] && numbers[1] != numbers[2] {
        return "scalene".to_string();
    }

    "none".to_string()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::type_of_triangle::triangle_type;

    #[test]
    fn equilateral() {
        assert_eq!("equilateral".to_string(), triangle_type(vec![1, 1, 1]))
    }

    #[test]
    fn isosceles() {
        assert_eq!("isosceles".to_string(), triangle_type(vec![9, 4, 9]))
    }

    #[test]
    fn scalene() {
        assert_eq!("scalene".to_string(), triangle_type(vec![3, 4, 5]))
    }

    #[test]
    fn none() {
        assert_eq!("none".to_string(), triangle_type(vec![5, 3, 8]));
        assert_eq!("none".to_string(), triangle_type(vec![8, 4, 4]));
    }
}
