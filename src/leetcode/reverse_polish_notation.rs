pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for token in tokens {
        match token.as_str() {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b)
            }
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b)
            }
            "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b)
            }
            "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b)
            }
            _ => stack.push(token.parse::<i32>().unwrap()),
        }
    }
    stack[0]
}

#[cfg(test)]
mod tests {
    use crate::string_vec;

    use super::*;

    #[test]
    fn evaluates_reverse_ploish_notation_expression() {
        assert_eq!(15, eval_rpn(string_vec!["5", "10", "+"]));
        assert_eq!(10, eval_rpn(string_vec!["5", "10", "+", "5", "-"]));
        assert_eq!(9, eval_rpn(string_vec!["2", "1", "+", "3", "*"]));
        assert_eq!(6, eval_rpn(string_vec!["4", "13", "5", "/", "+"]))
    }
}
