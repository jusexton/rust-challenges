use std::collections::HashMap;

struct Token {
    id: String,
    expires: i32,
}

impl Token {
    fn is_expired(&self, current_time: i32) -> bool {
        current_time >= self.expires
    }
}

struct AuthenticationManager {
    time_to_live: i32,
    tokens: HashMap<String, Token>,
}

impl AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        Self {
            time_to_live,
            tokens: HashMap::new(),
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        let new_token = Token {
            id: token_id.clone(),
            expires: current_time + self.time_to_live,
        };
        self.tokens.insert(token_id, new_token);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        // Only renew the token if it exists and is not expired

        if let Some(token) = self.tokens.get_mut(&token_id)
            && !token.is_expired(current_time)
        {
            token.expires = current_time + self.time_to_live
        }
    }

    fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
        self.tokens
            .iter()
            .filter(|entry| !entry.1.is_expired(current_time))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::auth_manager::{AuthenticationManager, Token};

    #[test_case(0, false)]
    #[test_case(4, false)]
    #[test_case(5, true)]
    #[test_case(10, true)]
    fn should_correctly_determine_if_token_is_expired(current_time: i32, should_expire: bool) {
        let token = Token {
            id: String::from("123"),
            expires: 5,
        };

        let is_expired = token.is_expired(current_time);
        assert_eq!(should_expire, is_expired)
    }

    #[test]
    fn should_generate_new_token() {
        let mut manager = AuthenticationManager::new(10);
        manager.generate(String::from("123"), 5);

        let token_count = manager.tokens.len();
        assert_eq!(1, token_count);
    }

    #[test]
    fn should_return_correct_number_of_unexpired_tokens() {
        let mut manager = AuthenticationManager::new(10);
        manager.generate(String::from("123"), 0);
        manager.generate(String::from("321"), 5);
        manager.generate(String::from("213"), 10);

        let count = manager.count_unexpired_tokens(10);
        assert_eq!(2, count);
    }

    #[test]
    fn should_renew_token() {
        let mut manager = AuthenticationManager::new(10);
        manager.generate(String::from("123"), 0);

        manager.renew(String::from("123"), 9);

        let count = manager.count_unexpired_tokens(15);
        assert_eq!(1, count);
    }
}
