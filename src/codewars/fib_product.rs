fn fib_product(product: u64) -> (u64, u64, bool) {
    let mut curr = 0;
    let mut next = 1;
    while curr * next < product {
        let new_next = curr + next;
        curr = next;
        next = new_next;
    }
    (curr, next, curr * next == product)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::fib_product::fib_product;

    #[test_case(4895, (55, 89, true))]
    #[test_case(5895, (89, 144, false))]
    fn test_fib_product(product: u64, expected: (u64, u64, bool)) {
        let actual = fib_product(product);
        assert_eq!(expected, actual)
    }
}
