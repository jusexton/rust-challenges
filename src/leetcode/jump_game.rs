pub fn can_jump(numbers: Vec<i32>) -> bool {
    let mut gas = 0;
    for number in numbers {
        if gas < 0 {
            return false;
        } else if number > gas {
            gas = number;
        }

        gas -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_if_goal_can_be_reached_by_jumping() {
        assert!(can_jump(vec![2, 3, 1, 1, 4]));
        assert!(can_jump(vec![0]))
    }
}
