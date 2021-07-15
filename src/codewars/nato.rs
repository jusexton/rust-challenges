fn to_nato(message: &str) -> String {
    return message
        .chars()
        .map(nato_from_character)
        .filter(|nato| !nato.is_empty())
        .collect::<Vec<String>>()
        .join(" ");
}

fn nato_from_character(character: char) -> String {
    let nato = match character.to_ascii_lowercase() {
        'a' => "Alfa",
        'b' => "Bravo",
        'c' => "Charlie",
        'd' => "Delta",
        'e' => "Echo",
        'f' => "Foxtrot",
        'g' => "Golf",
        'h' => "Hotel",
        'i' => "India",
        'j' => "Juliett",
        'k' => "Kilo",
        'l' => "Lima",
        'm' => "Mike",
        'n' => "November",
        'o' => "Oscar",
        'p' => "Papa",
        'q' => "Quebec",
        'r' => "Romeo",
        's' => "Sierra",
        't' => "Tango",
        'u' => "Uniform",
        'v' => "Victor",
        'w' => "Whiskey",
        'x' => "Xray",
        'y' => "Yankee",
        'z' => "Zulu",
        '.' => ".",
        '!' => "!",
        '?' => "?",
        _ => "",
    };
    String::from(nato)
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::nato::to_nato;

    #[test_case(
        "If you can read",
        "India Foxtrot Yankee Oscar Uniform Charlie Alfa November Romeo Echo Alfa Delta"
    )]
    #[test_case("go for it!", "Golf Oscar Foxtrot Oscar Romeo India Tango !")]
    fn should_return_correct_nato_translation(message: &str, expected: &str) {
        let actual = to_nato(message);
        let expected = String::from(expected);
        assert_eq!(actual, expected);
    }
}
