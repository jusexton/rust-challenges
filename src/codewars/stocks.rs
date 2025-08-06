use regex::Regex;

fn balance_statement(statement: &str) -> String {
    if statement.is_empty() {
        return String::from("Buy: 0 Sell: 0");
    }

    let records = statement.split(", ");
    let regex = Regex::new("^[A-Za-z0-9.]+\\s(0|[1-9][0-9]*)\\s((0|[1-9][0-9]*)\\.\\d+)\\s([SB])$")
        .unwrap();

    let mut buy = 0.0;
    let mut sell = 0.0;
    let mut invalid: Vec<&str> = vec![];
    for record in records {
        if let Some(captures) = regex.captures(record) {
            let units = captures.get(1).unwrap().as_str().parse::<f64>().unwrap();
            let price = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();
            let action = captures.get(4).unwrap().as_str();

            let total = units * price;
            println!("Total: {total}");
            if action == "B" {
                buy += total;
            } else {
                sell += total;
            }
        } else {
            invalid.push(record);
        }
    }
    println!("Sell: {sell}");

    let mistakes = if invalid.is_empty() {
        String::from("")
    } else {
        format!("; Badly formed {}: {} ;", invalid.len(), invalid.join(" ;"))
    };
    format!("Buy: {buy:.0} Sell: {sell:.0}{mistakes}")
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::stocks::balance_statement;

    #[test_case("", "Buy: 0 Sell: 0")]
    #[test_case(
        "CAP 1300 0.2 B, CLH16.NYM 50 56 S, OWW 1000 11 S, OGG 20 580.1 S",
        "Buy: 260 Sell: 11602; Badly formed 2: CLH16.NYM 50 56 S ;OWW 1000 11 S ;"
    )]
    #[test_case(
        "CLH16.NYM 67 450 S, BoAML 56 89.3 B, GOOG 45 5.5 B",
        "Buy: 5248 Sell: 0; Badly formed 1: CLH16.NYM 67 450 S ;"
    )]
    #[test_case(
        "GOOG 300 542.0 B, AAPL 50 145.0 B, CSCO 250.0 29 B, GOOG 200 580.0 S",
        "Buy: 169850 Sell: 116000; Badly formed 1: CSCO 250.0 29 B ;"
    )]
    #[test_case(
        "GOOG 90 160.45 B, JPMC 67 12.8 S, MYSPACE 24.0 210 B, CITI 50 450 B, CSCO 100 55.5 S",
        "Buy: 14440 Sell: 6408; Badly formed 2: MYSPACE 24.0 210 B ;CITI 50 450 B ;"
    )]
    #[test_case(
        "ZNGA 1300 2.66 B, CLH15.NYM 50 56.32 B, OWW 1000 11.623 B, OGG 20 580.1 B",
        "Buy: 29499 Sell: 0"
    )]
    #[test_case(
        "ZYGN 100 450 B, BoAML 24.0 34.8 B, CSCO 15 5.5 S, CLH16.NYM 90 210 P",
        "Buy: 0 Sell: 82; Badly formed 3: ZYGN 100 450 B ;BoAML 24.0 34.8 B ;CLH16.NYM 90 210 P ;"
    )]
    fn should_generate_correct_stock_report(statement: &str, expected: &str) {
        let expected = expected.to_string();
        let report = balance_statement(statement);
        assert_eq!(expected, report)
    }
}
