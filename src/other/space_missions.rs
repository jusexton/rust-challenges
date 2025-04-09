use std::io::BufRead;

pub fn longest_mars_mission<R>(reader: R) -> (u16, String)
where
    R: BufRead,
{
    let mut result = (0, "Unknown".to_string());
    for (idx, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|_| panic!("Could not read line: {idx}"));
        if line.starts_with('#') {
            continue;
        }

        // When the line is able to be split into 8 exact pieces we know we're
        // currently consuming a mission record line.
        let pieces: Vec<_> = line.split("|").map(str::trim).collect();
        if pieces.len() == 8 && pieces[2] == "Mars" {
            let duration = pieces[5]
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("Could not parse duration on line: {idx}"));
            if duration > result.0 {
                result = (duration, pieces[7].to_string());
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn ignores_comments() {
        assert_eq!(
            (0, "Unknown".to_string()),
            longest_mars_mission(Cursor::new(
                "# hello\n# more comments\n# 2048-09-07|  WXI-0590|Mars |   In Progress     |  6     | 380 |    69.57  |    ALP-780-GBT     "
            ))
        )
    }

    #[test]
    fn recognizes_values_with_leading_and_trailing_whitespace() {
        assert_eq!(
            (380, "ALP-780-GBT".to_string()),
            longest_mars_mission(Cursor::new(
                "2048-09-07|  WXI-0590|    Mars |   In Progress     |  6     | 380     |    69.57  |    ALP-780-GBT     "
            ))
        )
    }

    #[test]
    fn longest_mars_mission_is_captured() {
        assert_eq!(
            (420, "KLL-001-TND".to_string()),
            longest_mars_mission(Cursor::new(
                "2048-09-07|  WXI-0590|    Mars |   In Progress     |  6     | 380     |    69.57  |    ALP-780-GBT\n
                2041-09-13    |  EYO-5723     |     Moon|    Partial Success  |6   |   5000   |48.53    |   HRV-950-OIS\n
                2041-09-13    |  EYO-5723     |     Mars|    Partial Success  |    6   |   420   |48.53    |   KLL-001-TND"
            ))
        )
    }

    #[test]
    fn panics_at_invalid_duration() {
        let res = std::panic::catch_unwind(|| {
            longest_mars_mission(Cursor::new(
                "2048-09-07|  WXI-0590|    Mars |   In Progress     |  6     | abc     |    69.57  |    ALP-780-GBT",
            ))
        });
        assert!(res.is_err());
    }
}
