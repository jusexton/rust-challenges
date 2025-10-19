pub fn custom_sort_string(order: String, mut s: String) -> String {
    let order = order.char_indices().fold([0; 26], |mut acc, (idx, c)| {
        acc[c as usize - 'a' as usize] = idx;
        acc
    });
    unsafe {
        s.as_mut_vec()
            .sort_by_key(|c| order[*c as usize - 'a' as usize])
    };
    s
}

#[cfg(test)]
mod tests {
    use super::custom_sort_string;

    #[test]
    fn sorts_in_custom_order() {
        let order = "cba".to_string();
        let s = "abc".to_string();
        let result = custom_sort_string(order, s);
        assert_eq!("cba".to_string(), result)
    }
}
