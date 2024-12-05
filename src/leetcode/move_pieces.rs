pub fn can_change(start: String, target: String) -> bool {
    let n = start.len();
    let s_bytes = start.as_bytes();
    let t_bytes = target.as_bytes();

    let mut i = 0;
    let mut j = 0;
    while i < n || j < n {
        while i < n && s_bytes[i] == b'_' {
            i += 1;
        }
        while j < n && t_bytes[j] == b'_' {
            j += 1;
        }
        if i == n
            || j == n
            || s_bytes[i] != t_bytes[j]
            || (s_bytes[i] == b'L' && i < j)
            || (s_bytes[i] == b'R' && i > j)
        {
            break;
        }

        i += 1;
        j += 1;
    }
    i == n && j == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_if_target_can_be_reached_by_moving_pieces() {
        assert!(can_change("_L__R__R_".to_string(), "L______RR".to_string()));

        assert!(!can_change("_R".to_string(), "R_".to_string()));
        assert!(!can_change("_L_R__R_".to_string(), "R_____RR".to_string()));
    }
}
