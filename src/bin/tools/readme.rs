use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::anyhow;

use crate::util::parse_slug;

const README_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/README.md");
const LEETCODE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/leetcode");

/// A parsed section of the README, containing its direct bullet entries
/// and any named child sections (from sub-headers).
#[derive(Debug, Default)]
pub struct Section {
    pub entries: Vec<Entry>,
    headers: HashMap<String, Section>,
}

impl Section {
    pub fn header(&self, key: &str) -> Option<&Section> {
        self.headers.get(key)
    }

    pub fn path(&self, path: &[&str]) -> Option<&Section> {
        path.iter()
            .try_fold(self, |section, key| section.header(key))
    }
}

#[derive(Debug)]
pub struct Entry {
    pub text: String,
    pub line: usize,
}

struct Header<'a> {
    level: usize,
    text: &'a str,
}

fn parse_header(line: &str) -> Option<Header<'_>> {
    if !line.starts_with('#') {
        return None;
    }
    let trimmed = line.trim_start_matches('#');
    let level = line.len() - trimmed.len();
    let text = trimmed.trim_start();
    Some(Header { level, text })
}

fn parse_bullet(line: &str) -> Option<&str> {
    line.strip_prefix("- ")
}

fn navigate_to<'a>(root: &'a mut Section, path: &[(usize, String)]) -> &'a mut Section {
    let mut current = root;
    for (_, name) in path {
        current = current.headers.entry(name.clone()).or_default();
    }
    current
}

/// Parses the given README content into a tree of [`Section`]s that can be
/// navigated with chained `.header()` and `.path()` calls
pub fn parse(content: &str) -> Section {
    let mut root = Section::default();
    let mut path: Vec<(usize, String)> = Vec::new();

    for (idx, line) in content.lines().enumerate() {
        let line_num = idx + 1;

        if let Some(Header { level, text }) = parse_header(line) {
            path.retain(|(lvl, _)| *lvl < level);
            path.push((level, text.to_string()));
        } else if let Some(text) = parse_bullet(line) {
            let section = navigate_to(&mut root, &path);
            section.entries.push(Entry {
                text: text.to_string(),
                line: line_num,
            });
        }
    }

    root
}

pub fn verify() -> anyhow::Result<()> {
    let content = fs::read_to_string(README_PATH)?;
    let parsed = parse(&content);

    verify_leetcode(&parsed)?;

    Ok(())
}

fn verify_leetcode(root: &Section) -> anyhow::Result<()> {
    let leetcode = root
        .path(&[
            "Rust Programming Challenges",
            "Completed Programming Challenges",
            "LeetCode",
        ])
        .ok_or_else(|| anyhow!("Could not find LeetCode section."))?;

    let mut entries = HashSet::new();
    for difficulty in ["Easy", "Medium", "Hard"] {
        let res = get_leetcode_entry_names(leetcode, difficulty)?;
        entries.extend(res.collect::<anyhow::Result<Vec<_>>>()?);
    }

    let missing: Vec<_> = challenge_files(LEETCODE_DIR)?
        .into_iter()
        .filter(|file| !entries.contains(file.as_str()))
        .collect();

    if !missing.is_empty() {
        let list = missing.join("\n  ");
        return Err(anyhow!(
            "The README file is missing the corresponding challenge entries:\n  {list}"
        ));
    }

    Ok(())
}

fn get_leetcode_entry_names(
    section: &Section,
    header: &str,
) -> anyhow::Result<impl Iterator<Item = anyhow::Result<String>>> {
    let section = section
        .header(header)
        .ok_or_else(|| anyhow!("Could not find LeetCode '{header}' section."))?;
    Ok(section
        .entries
        .iter()
        .map(|e| parse_leetcode_entry_name(&e.text)))
}

fn parse_leetcode_entry_name(entry_text: &str) -> anyhow::Result<String> {
    let url = entry_text
        .split("](")
        .nth(1)
        .and_then(|s| s.trim_end().strip_suffix(')'))
        .unwrap_or("");
    let slug = parse_slug(url, "/problems/")
        .ok_or_else(|| anyhow!("Unable to parse leetcode challenge slub from {entry_text}"))?;
    Ok(slug.replace("-", "_"))
}

fn challenge_files(dir: &str) -> anyhow::Result<Vec<String>> {
    let mut slugs = fs::read_dir(dir)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let name = path.file_stem()?.to_str()?.to_string();
            // skip mod.rs and any non-.rs files
            if path.extension()?.to_str()? == "rs" && name != "mod" {
                Some(name)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    slugs.sort();
    Ok(slugs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
# Root

## Section A

- entry a1
- entry a2

### Section A1

- entry a1_1

## Section B

- entry b1
";

    #[test]
    fn navigates_to_nested_section() {
        let parsed = parse(SAMPLE);
        let entries = &parsed
            .path(&["Root", "Section A", "Section A1"])
            .unwrap()
            .entries;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].text, "entry a1_1");
    }

    #[test]
    fn captures_line_numbers() {
        let parsed = parse(SAMPLE);
        let entries = &parsed.path(&["Root", "Section A"]).unwrap().entries;
        assert_eq!(entries[0].line, 5);
        assert_eq!(entries[1].line, 6);
    }

    #[test]
    fn sibling_sections_are_independent() {
        let parsed = parse(SAMPLE);
        let root = parsed.header("Root").unwrap();
        assert!(root.header("Section A").is_some());
        assert!(root.header("Section B").is_some());
        let b_entries = &root.header("Section B").unwrap().entries;
        assert_eq!(b_entries.len(), 1);
        assert_eq!(b_entries[0].text, "entry b1");
    }
}
