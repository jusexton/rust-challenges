use regex::Regex;

fn get_order(input: String) -> String {
    let re = Regex::new("burger|fries|chicken|pizza|sandwich|onionrings|milkshake|coke").unwrap();
    let mut matches = re
        .find_iter(&input)
        .map(|m| m.as_str())
        .map(|s| [&s[..1].to_uppercase(), &s[1..]].join(""))
        .collect::<Vec<String>>();

    matches.sort_by_key(|item| sort_order(item));

    return matches.join(" ");
}

fn sort_order(item: &String) -> u8 {
    let item = item.as_str();
    match item {
        "Burger" => 0,
        "Fries" => 1,
        "Chicken" => 2,
        "Pizza" => 3,
        "Sandwich" => 4,
        "Onionrings" => 5,
        "Milkshake" => 6,
        "Coke" => 7,
        _ => u8::MAX,
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::order::get_order;

    #[test_case(
        "milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza",
        "Burger Fries Chicken Pizza Pizza Pizza Sandwich Milkshake Milkshake Coke"
    )]
    #[test_case(
        "pizzachickenfriesburgercokemilkshakefriessandwich",
        "Burger Fries Fries Chicken Pizza Sandwich Milkshake Coke"
    )]
    fn should_return_the_string_with_spaces_between_title_cased_menu_items(
        input: &str,
        expected: &str,
    ) {
        let actual = get_order(input.to_string());
        assert_eq!(expected, actual);
    }
}
