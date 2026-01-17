// na pasta main/src/bank/accounts.rs
#[derive(Debug)]
pub struct Account {
    pub owner: String,
    pub balance: i32,
}

impl Account {
    pub fn new(owner: &str) -> Self {
        Account {
            owner: String::from(owner),
            balance: 0,
        }
    }
}

// ---
// na pasta main/src/bank/transactions.rs
use crate::bank::accounts::Account;
pub fn deposit(acc: &mut Account, amount: i32) {
    acc.balance += amount;
    println!(
        "[TRANSACTION] Deposited ${}. New balance: ${}",
        amount, acc.balance
    );
}

pub fn withdraw(acc: &mut Account, amount: i32) {
    if amount > acc.balance {
        println!(
            "[TRANSACTION] ERROR: Insufficient funds for ${} withdrawl.",
            amount
        );
    } else {
        acc.balance -= amount;
        println!(
            "[TRANSACTION] Withdraw ${}. New balance: ${}",
            amount, acc.balance
        );
    }
}

// ---
// na pasta main/src/bank.rs
pub mod accounts;
pub mod transactions;

pub fn announce(message: &str) {
    println!("[BANK ANNOUNCEMENT] {message}");
}

// ---
// na pasta main/src/main.rs
mod bank;

fn main( {
    let mut acc: Account = bank::accounts::Account::new("Bob");
    println!("[ACCOUNT] Created: {:?}", acc);

    bank::transactions::deposit(&mut acc, 150);
    bank::transactions::withdraw(&mut acc, 20);
    println!("[ACCOUNT] Final state: {:?}", acc);
    bank::announce("Maintenance at 01:30pm!")
})