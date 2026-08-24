use std::{fmt, fs, path::Path};

use anyhow::{anyhow, bail};
use serde::Deserialize;

use crate::{readme, util};

const README_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/README.md");
const ROMAN_NUMERALS: &[&str] = &["i", "ii", "iii", "iv"];
const MINOR_WORDS: &[&str] = &[
    "a", "an", "the", "and", "but", "or", "nor", "for", "yet", "so", "at", "by", "in", "of", "on",
    "to", "up", "as", "it", "its", "with",
];

#[derive(Debug, Deserialize)]
enum LeetCodeDifficulty {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for LeetCodeDifficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LeetCodeDifficulty::Easy => "Easy",
                LeetCodeDifficulty::Medium => "Medium",
                LeetCodeDifficulty::Hard => "Hard",
            }
        )
    }
}

pub fn scaffold_new_challenge(url: &str) -> anyhow::Result<()> {
    let host = util::extract_host(url).ok_or_else(|| anyhow!("could not parse host from url"))?;

    match host {
        "leetcode.com" => scaffold_leetcode_challenge(url)?,
        _ => bail!("unsupported host: {host}"),
    }

    Ok(())
}

fn scaffold_leetcode_challenge(url: &str) -> anyhow::Result<()> {
    let name = util::parse_slug(url, "/problems/")
        .ok_or_else(|| anyhow!("Could not parse problem slug from URL."))?;

    let snake_name = name.replace("-", "_");
    let file_path = format!("src/leetcode/{snake_name}.rs");
    if Path::new(&file_path).exists() {
        bail!("{file_path} already exists");
    }

    fs::write(&file_path, template(&snake_name))?;
    println!("Created: {file_path}");

    if let Ok(difficulty) = get_difficulty(name) {
        let title = to_title_case(name, "-");
        let minimum_url = format!("https://leetcode.com/problems/{}", name);
        insert_readme_line(
            &[
                "Rust Programming Challenges",
                "Completed Programming Challenges",
                "LeetCode",
                &difficulty.to_string(),
            ],
            &format!("- [{title}]({minimum_url})"),
        )?;

        insert_module_item(&snake_name, "src/leetcode/mod.rs")?;
    } else {
        println!(
            "Warning: Was not able to retrieve difficulty using leetcode API. Skipping README update."
        )
    }

    Ok(())
}

fn template(name: &str) -> String {
    format!(
        r#"pub fn {name}() {{
    todo!()
}}

#[cfg(test)]
mod tests {{
    use super::*;
}}
"#
    )
}

fn get_difficulty(slug: &str) -> anyhow::Result<LeetCodeDifficulty> {
    #[derive(Deserialize)]
    struct Response {
        data: ResponseData,
    }

    #[derive(Deserialize)]
    struct ResponseData {
        question: ResponseQuestion,
    }

    #[derive(Deserialize)]
    struct ResponseQuestion {
        difficulty: LeetCodeDifficulty,
    }

    let body = serde_json::json!({
        "query": format!("query {{ question(titleSlug: \"{slug}\") {{ difficulty }} }}")
    });

    let response: Response = reqwest::blocking::Client::new()
        .post("https://leetcode.com/graphql")
        .json(&body)
        .send()?
        .json()?;

    Ok(response.data.question.difficulty)
}

fn insert_readme_line(path: &[&str], entry_text: &str) -> anyhow::Result<()> {
    let mut content = fs::read_to_string(README_PATH)?;

    let parsed = readme::parse(&content);

    let section = parsed
        .path(path)
        .ok_or_else(|| anyhow!("Could not properly parse path to readme section."))?;
    let line_number = section.entries.iter().map(|e| e.line).max().unwrap_or(0);
    let offset = line_offset(&content, line_number)
        .ok_or_else(|| anyhow!("Readme entry insertion line could not be found."))?;

    content.insert_str(offset, &format!("{}\n", entry_text));

    fs::write(README_PATH, content)?;

    Ok(())
}

fn to_title_case(input: &str, delimiter: &str) -> String {
    input
        .split(delimiter)
        .enumerate()
        .map(|(i, word)| {
            let lower = word.to_lowercase();
            if ROMAN_NUMERALS.contains(&lower.as_str()) {
                lower.to_uppercase()
            } else if i == 0 || !MINOR_WORDS.contains(&lower.as_str()) {
                capitalize(&lower)
            } else {
                lower
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
    }
}

fn line_offset(content: &str, line_number: usize) -> Option<usize> {
    if line_number == 0 {
        return Some(0);
    }
    content
        .char_indices()
        .filter(|(_, c)| *c == '\n')
        .nth(line_number - 1)
        .map(|(i, _)| i + 1)
}

fn insert_module_item(module_name: &str, mod_file_path: &str) -> anyhow::Result<()> {
    let mut content = fs::read_to_string(mod_file_path)?;
    let mod_item = format!("mod {};\n", module_name);

    let line_number = find_module_insertion_line(&content, &mod_item);
    let offset = line_offset(&content, line_number)
        .ok_or_else(|| anyhow!("Entry insertion line could not be found."))?;

    content.insert_str(offset, &mod_item);

    fs::write(mod_file_path, content)?;

    Ok(())
}

fn find_module_insertion_line(content: &str, mod_item: &str) -> usize {
    let mod_item = mod_item.trim_end_matches('\n');
    content
        .lines()
        .enumerate()
        .find_map(|(line_number, line)| match mod_item.cmp(line) {
            std::cmp::Ordering::Less => Some(line_number),
            _ => None,
        })
        .unwrap_or(content.lines().count())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test]
    fn find_module_insertion_line_empty_content() {
        assert_eq!(find_module_insertion_line("", "mod aaa;\n"), 0);
    }

    #[test]
    fn find_module_insertion_line_inserts_before_first_when_new_module_comes_first() {
        let content = "mod bbb;\nmod ccc;\n";
        assert_eq!(find_module_insertion_line(content, "mod aaa;\n"), 0);
    }

    #[test]
    fn find_module_insertion_line_inserts_in_middle() {
        let content = "mod aaa;\nmod ccc;\n";
        assert_eq!(find_module_insertion_line(content, "mod bbb;\n"), 1);
    }

    #[test]
    fn find_module_insertion_line_inserts_at_end_when_new_module_comes_last() {
        let content = "mod aaa;\nmod bbb;\n";
        assert_eq!(find_module_insertion_line(content, "mod zzz;\n"), 2);
    }

    #[test]
    fn find_module_insertion_line_inserts_before_sole_entry_that_comes_after() {
        let content = "mod zzz;\n";
        assert_eq!(find_module_insertion_line(content, "mod aaa;\n"), 0);
    }

    #[test]
    fn find_module_insertion_line_inserts_after_sole_entry_that_comes_before() {
        let content = "mod aaa;\n";
        assert_eq!(find_module_insertion_line(content, "mod zzz;\n"), 1);
    }

    #[test]
    fn find_module_insertion_line_respects_underscore_ordering() {
        let content = "mod foo;\nmod goo;\n";
        assert_eq!(find_module_insertion_line(content, "mod foo_bar;\n"), 1);
    }

    #[test_case("the quick brown fox", "The Quick Brown Fox")]
    #[test_case("a tale of two cities", "A Tale of Two Cities")]
    #[test_case("to kill a mockingbird", "To Kill a Mockingbird")]
    #[test_case("gone with the wind", "Gone with the Wind")]
    #[test_case("the lord of the rings", "The Lord of the Rings")]
    #[test_case("it", "It")]
    #[test_case("hello world", "Hello World")]
    #[test_case("halo ii anniversary", "Halo II Anniversary")]
    #[test_case("final fantasy iv", "Final Fantasy IV")]
    fn should_convert_to_title_case(input: &str, expected: &str) {
        assert_eq!(to_title_case(input, " "), expected);
    }
}
