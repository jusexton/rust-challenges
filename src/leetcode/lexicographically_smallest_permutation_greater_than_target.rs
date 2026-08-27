pub fn lex_greater_permutation(s: String, target: String) -> String {
    let mut count = [0i32; 26];
    let target_bytes = target.as_bytes();

    for &b in s.as_bytes() {
        count[(b - b'a') as usize] += 1;
    }
    for &b in target_bytes {
        count[(b - b'a') as usize] -= 1;
    }

    for i in (0..target.len()).rev() {
        let cur = (target_bytes[i] - b'a') as usize;
        count[cur] += 1;

        if count.iter().any(|&x| x < 0) {
            continue;
        }

        let smallest = (cur + 1..26).find(|&c| count[c] > 0);
        let Some(smallest) = smallest else { continue };

        count[smallest] -= 1;
        let mut ans = target[..i].to_string();
        ans.push((b'a' + smallest as u8) as char);

        for c in 0..26 {
            for _ in 0..count[c] {
                ans.push((b'a' + c as u8) as char);
            }
        }

        return ans;
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_greater_permutation() {
        assert_eq!(
            "bca".to_string(),
            lex_greater_permutation("abc".to_string(), "bba".to_string())
        );
    }
}
