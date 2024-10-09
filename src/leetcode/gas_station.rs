pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total_surplus = 0;
    let mut gas_level = 0;
    let mut start = 0;

    for idx in 0..gas.len() {
        total_surplus += gas[idx] - cost[idx];
        gas_level += gas[idx] - cost[idx];

        if gas_level < 0 {
            gas_level = 0;
            start = idx + 1;
        }
    }

    match total_surplus < 0 {
        true => -1,
        false => start as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::can_complete_circuit;

    #[test]
    fn returns_correct_starting_station() {
        assert_eq!(
            3,
            can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
        )
    }

    #[test]
    fn returns_negative_one_when_circuit_can_not_be_completed() {
        assert_eq!(-1, can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]))
    }
}
