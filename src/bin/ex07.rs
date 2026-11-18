//! # Exercise 07 — BankAccount struct
//!
//! **Goal** — model a bank account with data **and** behaviour.
//!
//! ```text
//! struct BankAccount { owner: String, balance: f64 }
//! ```
//!
//! | Method     | Signature                                              |
//! |------------|--------------------------------------------------------|
//! | `new`      | `fn new(owner: &str, balance: f64) -> Self`            |
//! | `deposit`  | `fn deposit(&mut self, amount: f64)`                   |
//! | `withdraw` | `fn withdraw(&mut self, amount: f64) -> Result<(), String>` |
//! | `display`  | `fn display(&self)`                                    |
//!
//! **Run**  `npm run ex -- ex07`
//! **Test** `npm run test:ex -- ex07`
//!
//! **Expected output**
//!
//! ```text
//! Aidan: 100.00 CHF
//! Aidan: 150.00 CHF
//! error: insufficient funds (balance 150.00, requested 500.00)
//! Aidan: 150.00 CHF
//! ```
//!
//! **Hints**
//! - `display` only reads → `&self`; the others change the balance → `&mut self`
//! - `withdraw` returns `Err(...)` when `amount > self.balance`
//! - `format!` builds a `String` (like `println!`, but returns it)
//! - `Self` inside an `impl` block means the type being implemented

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: &str, balance: f64) -> Self {
        Self {
            owner: owner.to_string(),
            balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        let _ = amount;
        // TODO: self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        let _ = amount;
        Err("TODO: implement withdraw".to_string())
    }

    fn display(&self) {
        println!("{}: {:.2} CHF", self.owner, self.balance);
    }
}

fn main() {
    let mut account = BankAccount::new("Aidan", 100.0);

    account.display();
    account.deposit(50.0);
    account.display();

    match account.withdraw(500.0) {
        Ok(()) => println!("withdrawal ok"),
        Err(e) => println!("error: {e}"),
    }

    account.display();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "exercise not solved yet — run: npm run test:ex -- ex07"]
    fn deposit_and_withdraw() {
        let mut account = BankAccount::new("Test", 100.0);

        account.deposit(50.0);
        assert_eq!(account.balance, 150.0);

        assert!(account.withdraw(500.0).is_err());
        assert_eq!(account.balance, 150.0);

        assert!(account.withdraw(50.0).is_ok());
        assert_eq!(account.balance, 100.0);
    }
}
