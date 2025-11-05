struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if !self.valid_account(account1) || !self.valid_account(account2) {
            return false;
        }

        let balance = self.balance[account1 as usize - 1];
        if balance >= money {
            self.balance[account1 as usize - 1] = balance - money;
            self.balance[account2 as usize - 1] += money;
            true
        } else {
            false
        }
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if !self.valid_account(account) {
            return false;
        }
        self.balance[account as usize - 1] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if !self.valid_account(account) {
            return false;
        }
        let balance = self.balance[account as usize - 1];
        if balance < money {
            return false;
        }
        self.balance[account as usize - 1] -= money;
        true
    }

    fn valid_account(&self, account: i32) -> bool {
        account > 0 && account <= self.balance.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::simple_bank_system::Bank;

    #[test]
    fn test_transactions() {
        let mut b = Bank::new(vec![10, 100, 20, 50, 30]);
        assert!(b.withdraw(3, 10));
        assert!(b.transfer(5, 1, 20));
        assert!(b.deposit(5, 20));
        assert!(!b.transfer(3, 4, 15));
        assert!(!b.withdraw(10, 50));
    }
}
