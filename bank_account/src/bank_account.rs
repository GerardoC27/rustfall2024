#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*; // This imports the BankAccount struct and its methods for testing

    #[test]
    fn test_new_account() {
        // Test for creating a new BankAccount with an initial balance
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        // Test depositing money into the account
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);

        // Test negative deposit (should be ignored)
        account.deposit(-10.0);
        assert_eq!(account.balance(), 150.0);
    }

    #[test]
    fn test_withdraw() {
        // Test withdrawing money from the account
        let mut account = BankAccount::new(100.0);
        account.withdraw(50.0);
        assert_eq!(account.balance(), 50.0);

        // Test trying to withdraw more money than available (should be ignored)
        account.withdraw(60.0);
        assert_eq!(account.balance(), 50.0);

        // Test negative withdrawal (should be ignored)
        account.withdraw(-10.0);
        assert_eq!(account.balance(), 50.0);
    }
}
