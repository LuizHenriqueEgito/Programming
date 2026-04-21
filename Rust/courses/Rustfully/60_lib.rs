/*
Não é preciso criar a pasta bank e fora dela o arquivo bank.rs
basta criar a pasta bank e o que você colocaria no arquivo bank.rs fora da pasta bank
você coloca esse arquivo bank.rs dentro da pasta bank e altera o nome dele para mod.rs
*/

/*
No mesmo diretorio do seu arquivo main.rs
você pode criar o arquivo lib.rs
*/
// lib.rs
pub mod banking {
    pub mod accounts {
        #[derive(Debug)]
        pub struct Account {
            pub account_number: u32,
            pub balance: f64,
        }

        pub fn open_account(id: i32) -> Account {
            println!("Account {} opned!", id);
            Account {
                account_number: id,
                balance: 0.0,
            }
        }

        #[allow(dead_code)]
        fn close_account(account: &mut Account) {
            println!("Account {} closed", account.account_number);
            account.balance = 0.0
        }
    }

    pub mod transactions {
        use super::accounts::Account;

        pub fn deposit(account: &mut Account, amount: f64) {
            account.balance += amount;
            println!(
                "Deposited ${:?} into account {}. New balance: ${:?}",
                amount, account.account_number, account.balance
            );
        }
    }
}

// no main.rs
use main::banking::{accounts, transactions};

fn main() {
    let mut james: Account = accounts::open_account(1);
    let mut bob: Account = accounts::open_account(2);

    transactions::deposit(&mut james, 200.0);
    transactions::deposit(&mut bob, 120.0);
}