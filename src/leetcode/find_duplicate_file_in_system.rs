use std::collections::HashMap;

pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
    let content_groups: HashMap<&str, Vec<String>> =
        paths.iter().fold(HashMap::new(), |mut acc, curr| {
            let mut pieces = curr.split_ascii_whitespace();
            let dir = pieces.next().unwrap();
            for file in pieces {
                let file: Vec<&str> = file.split('(').collect();
                let file_group: &mut Vec<String> = acc.entry(file[1]).or_default();
                file_group.push(format!("{}/{}", dir, file[0]));
            }
            acc
        });

    content_groups
        .into_values()
        .filter(|group| group.len() > 1)
        .collect()
}
