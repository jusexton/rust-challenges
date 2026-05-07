pub fn convert(s: String, num_rows: i32) -> String {
    fn append(o: &mut String, bytes: &[u8], start: usize, step: usize) {
        let mut i = 0;
        while let Some(&c) = bytes.get(start + i * step) {
            o.push(c as char);
            i += 1;
        }
    }

    let num_rows = num_rows as usize;
    let bytes = s.as_bytes();
    let step = match num_rows {
        1 => 1,
        n => n * 2 - 2,
    };
    let mut o = String::with_capacity(s.len());

    // Top row
    append(&mut o, bytes, 0, step);

    // Middle rows
    for row in 1..num_rows.saturating_sub(1) {
        let mut i = 0;
        while let Some(&a) = bytes.get(row + i * step) {
            o.push(a as char);
            match bytes.get((step - row) + i * step).copied() {
                Some(b) => o.push(b as char),
                None => break,
            }
            i += 1;
        }
    }

    // Bottom row
    if num_rows > 1 {
        append(&mut o, bytes, num_rows - 1, step);
    }

    o
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
        );

        assert_eq!("A".to_string(), convert("A".to_string(), 1));
    }
}
