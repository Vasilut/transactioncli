
use std::collections::HashMap;

use crate::entities::{ClientAccount, Transaction};

pub struct PaymentEngine {
    accounts: HashMap<u16, ClientAccount>, 
    transactions: HashMap<u32, (u16, f64, bool)>,
}

impl PaymentEngine {
    pub fn new() -> Self {
        PaymentEngine {
            accounts : HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn process_transaction(&mut self, transaction : Transaction) {
        match transaction.r#type.as_str() {
            "deposit" => self.deposit(transaction.tx, transaction.client, transaction.amount),
            "withdrawal" => self.withdraw( transaction.client, transaction.amount),
            _ => (),
        }
    }

    fn deposit(&mut self, transaction_id : u32, client_id : u16, amount: f64) {

        let current_account = self.accounts.entry(client_id).or_insert(ClientAccount {
            client: client_id,
            available_amount: 0.0,
            held_amount : 0.0,
            total_amount: 0.0,
            locked : false,
        });

        if current_account.locked {
            return;
        }

        current_account.available_amount += amount;
        current_account.total_amount += amount;

        self.transactions.insert(transaction_id, (client_id, amount, false));

    }

    fn withdraw(&mut self, client_id : u16, amount: f64) {

        if let Some(current_account) = self.accounts.get_mut(&client_id) {
            if current_account.locked || current_account.available_amount < amount {
                return;
            }

            current_account.available_amount -= amount;
            current_account.total_amount -= amount;
        }

    } 

    /* 
    fn dispute(&mut self, transaction_id : u32, client_id : u16) {

    }

    fn resolve(&mut self, transaction_id : u32, client_id : u16) {

    }

    fn chargeback(&mut self, transaction_id : u32, client_id : u16) {

    }
    */

    pub fn get_accounts(&mut self) -> Vec<ClientAccount> {
        self.accounts.values().cloned().collect()
    }

}