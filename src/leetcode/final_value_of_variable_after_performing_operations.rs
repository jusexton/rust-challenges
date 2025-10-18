pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.into_iter().fold(0, |acc, op| {
        acc + match op.as_str() {
            "X++" | "++X" => 1,
            "X--" | "--X" => -1,
            _ => 0,
        }
    })
}
