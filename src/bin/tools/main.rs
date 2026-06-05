mod readme;

use serde::Deserialize;

use std::{
    fmt,
    fs::{self},
    path::Path,
};

use anyhow::{anyhow, bail};
use clap::{Parser, Subcommand};

const ROMAN_NUMERALS: &[&str] = &["i", "ii", "iii", "iv"];
const MINOR_WORDS: &[&str] = &[
    "a", "an", "the", "and", "but", "or", "nor", "for", "yet", "so", "at", "by", "in", "of", "on",
    "to", "up", "as", "it", "its", "with",
];
const README_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/README.md");

#[derive(Debug, Deserialize)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Difficulty::Easy => "Easy",
                Difficulty::Medium => "Medium",
                Difficulty::Hard => "Hard",
            }
        )
    }
}

#[derive(Parser)]
#[clap(name = "tools", version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone)]
enum Command {
    New {
        #[clap(long)]
        url: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::New { url } => scaffold_new_challenge(&url),
    }
}

fn scaffold_new_challenge(url: &str) -> anyhow::Result<()> {
    let host = extract_host(url).ok_or_else(|| anyhow!("could not parse host from url"))?;

    match host {
        "leetcode.com" => scaffold_leetcode_challenge(url)?,
        _ => bail!("unsupported host: {host}"),
    }

    Ok(())
}

fn scaffold_leetcode_challenge(url: &str) -> anyhow::Result<()> {
    let name = parse_slug(url, "/problems/")
        .ok_or_else(|| anyhow!("Could not parse problem slug from URL."))?;

    let snake_name = name.replace("-", "_");
    let file_path = format!("src/leetcode/{snake_name}.rs");
    if Path::new(&file_path).exists() {
        bail!("{file_path} already exists");
    }

    fs::write(&file_path, template(&snake_name))?;
    println!("Created: {file_path}");

    let difficulty = get_difficulty(name)?;
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

    Ok(())
}

fn extract_host(url: &str) -> Option<&str> {
    let after_scheme = url.split("://").nth(1).unwrap_or(url);
    let host_and_port = after_scheme.split('/').next()?;
    let host = host_and_port.split(':').next()?;
    Some(host)
}

fn parse_slug<'a>(url: &'a str, res: &str) -> Option<&'a str> {
    let rest = url.split(res).nth(1)?;
    let slug = rest.split('/').next()?;
    if slug.is_empty() { None } else { Some(slug) }
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

fn get_difficulty(slug: &str) -> anyhow::Result<Difficulty> {
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
        difficulty: Difficulty,
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
        .ok_or_else(|| anyhow!("Entry insertion line could not be found found."))?;

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

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

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
