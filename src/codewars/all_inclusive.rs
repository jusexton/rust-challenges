fn contain_all_rots(s: &str, rotations: Vec<&str>) -> bool {
    (0..s.len())
        .map(|index| s[index..].to_owned() + &s[..index])
        .all(|x| rotations.contains(&x.as_str()))
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::all_inclusive::contain_all_rots;

    #[test_case("bsjq", &["bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"], true)]
    #[test_case("Ajylvpy", &["Ajylvpy", "ylvpyAj", "jylvpyA", "lvpyAjy", "pyAjylv", "vpyAjyl", "ipywee"], false)]
    fn should_correctly_return_whether_all_of_given_strings_rotations_are_in_given_vector(
        s: &str,
        rotations: &[&str],
        expected: bool,
    ) {
        let actual = contain_all_rots(s, rotations.to_vec());
        assert_eq!(expected, actual);
    }
}
