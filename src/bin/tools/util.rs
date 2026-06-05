pub fn extract_host(url: &str) -> Option<&str> {
    let after_scheme = url.split("://").nth(1).unwrap_or(url);
    let host_and_port = after_scheme.split('/').next()?;
    let host = host_and_port.split(':').next()?;
    Some(host)
}

pub fn parse_slug<'a>(url: &'a str, res: &str) -> Option<&'a str> {
    let rest = url.split(res).nth(1)?;
    let slug = rest.split('/').next()?;
    if slug.is_empty() { None } else { Some(slug) }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("https://leetcode.com/problems/two-sum", Some("leetcode.com"))]
    #[test_case("https://leetcode.com/", Some("leetcode.com"))]
    #[test_case("https://localhost:8080/problems/two-sum", Some("localhost"))]
    #[test_case("leetcode.com/problems/two-sum", Some("leetcode.com"))]
    fn extract_host_returns_host(url: &str, expected: Option<&str>) {
        assert_eq!(extract_host(url), expected);
    }

    #[test_case("https://leetcode.com/problems/two-sum", "/problems/", Some("two-sum"); "returns slug")]
    #[test_case("https://leetcode.com/problems/two-sum/", "/problems/", Some("two-sum"); "ignores trailing slash")]
    #[test_case("https://leetcode.com/problems/two-sum/submissions", "/problems/", Some("two-sum"); "ignores sub-paths")]
    #[test_case("https://leetcode.com/problems/", "/problems/", None; "returns none for empty slug")]
    #[test_case("https://leetcode.com/other/two-sum", "/problems/", None; "returns none when separator is absent")]
    fn parse_slug_returns_slug(url: &str, separator: &str, expected: Option<&str>) {
        assert_eq!(parse_slug(url, separator), expected);
    }
}
