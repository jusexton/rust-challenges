fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

pub fn convert(s: String, n: i32) -> String {
    if n == 1 || n as usize >= s.len() {
        return s;
    }

    let mut rows = vec![String::new(); n as usize];

    let mut row_index = 0;
    let mut step = 1;
    for c in s.chars() {
        rows[row_index].push(c);
        if row_index == 0 {
            step = 1;
        } else if row_index == n as usize - 1 {
            step = -1;
        }

        row_index = add(row_index, step).unwrap();
    }

    rows.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_string_to_zigzag() {
        assert_eq!(
            "PAHNAPLSIIGYIR".to_string(),
            convert("PAYPALISHIRING".to_string(), 3)
        );

        assert_eq!(
            "PINALSIGYAHRPI".to_string(),
            convert("PAYPALISHIRING".to_string(), 4)
        )
    }
}
