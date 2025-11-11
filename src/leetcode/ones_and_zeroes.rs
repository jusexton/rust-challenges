use std::collections::HashMap;

pub fn find_max_form(strings: Vec<String>, m: i32, n: i32) -> i32 {
    let mut dp: HashMap<(i32, i32), i32> = HashMap::new();
    dp.insert((0, 0), 0);

    for s in strings {
        let (zeroes, ones) = s.chars().fold((0, 0), |mut acc, curr| {
            match curr {
                '0' => acc.0 += 1,
                '1' => acc.1 += 1,
                c => panic!("unexpected char: {c}"),
            }
            acc
        });

        let updates = dp.iter().fold(
            HashMap::new(),
            |mut acc, ((prev_zeroes, prev_ones), count)| {
                let (new_zeroes, new_ones) = (prev_zeroes + zeroes, prev_ones + ones);
                if new_zeroes <= m && new_ones <= n {
                    match dp.get(&(new_zeroes, new_ones)) {
                        Some(&prev_count) if prev_count < count + 1 => {
                            acc.insert((new_zeroes, new_ones), count + 1);
                        }
                        None => {
                            acc.insert((new_zeroes, new_ones), count + 1);
                        }
                        _ => {}
                    };
                }
                acc
            },
        );

        for (ones_and_zeroes, count) in updates {
            dp.insert(ones_and_zeroes, count);
        }
    }

    dp.into_values()
        .max()
        .expect("Expected dp map to at least have zero.")
}

#[cfg(test)]
mod tests {
    use crate::{leetcode::ones_and_zeroes::find_max_form, string_vec};

    #[test]
    fn test_find_max_form() {
        assert_eq!(
            4,
            find_max_form(string_vec!["10", "0001", "111001", "1", "0"], 5, 3)
        );

        assert_eq!(2, find_max_form(string_vec!["10", "0", "1"], 1, 1));
    }
}
