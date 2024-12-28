use std::collections::HashMap;

pub fn group_anagrams(strings: Vec<String>) -> Vec<Vec<String>> {
    strings
        .into_iter()
        .fold(HashMap::<[u8; 26], Vec<String>>::new(), |mut acc, s| {
            let freqs = s
                .bytes()
                .map(|b| (b - b'a') as usize)
                .fold([0; 26], |mut freqs, bin| {
                    freqs[bin] += 1;
                    freqs
                });
            acc.entry(freqs).or_default().push(s);
            acc
        })
        .into_values()
        .collect()
}
