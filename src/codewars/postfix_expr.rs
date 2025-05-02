fn postfix_evaluator(expr: &str) -> i64 {
    let mut stack = vec![];
    for element in expr.split_ascii_whitespace() {
        match element.parse::<i64>() {
            Ok(number) => stack.push(number),
            Err(_) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let calc = match element {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => unreachable!(),
                };
                stack.push(calc);
            }
        }
    }

    match stack.first() {
        Some(result) => *result,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_expression() {
        assert_eq!(0, postfix_evaluator(""))
    }

    #[test]
    fn simple_expression() {
        assert_eq!(65, postfix_evaluator("20 45 +"))
    }

    #[test]
    fn complex_expression() {
        assert_eq!(10, postfix_evaluator("2 3 9 4 / + *"));
    }
}
