
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
            "deposit" => self.deposit(transaction.tx, transaction.client, transaction.amount.unwrap_or(0.0)),
            "withdrawal" => self.withdraw( transaction.tx, transaction.client, transaction.amount.unwrap_or(0.0)),
            "dispute" => self.dispute(transaction.tx, transaction.client),
            "resolve" => self.resolve(transaction.tx, transaction.client),
            "chargeback" => self.chargeback(transaction.tx, transaction.client),
            _ => eprintln!("Unknown transaction type: {}", transaction.r#type),
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

    fn withdraw(&mut self, transaction_id : u32, client_id : u16, amount: f64) {

        if let Some(current_account) = self.accounts.get_mut(&client_id) {
            if current_account.locked || current_account.available_amount < amount {
                return;
            }

            current_account.available_amount -= amount;
            current_account.total_amount -= amount;

            self.transactions.insert(transaction_id, (client_id, amount, false));
        }
    } 

    
    fn dispute(&mut self, transaction_id : u32, client_id : u16) {

        if let Some(transaction) = self.transactions.get(&transaction_id) {
            let (transaction_client_id, amount, _) = (transaction.0, transaction.1, transaction.2);

            if transaction_client_id == client_id {
                if let Some(account) = self.accounts.get_mut(&client_id) {
                    account.available_amount -= amount;
                    account.held_amount += amount;
                }
            }
        }
    }

    fn resolve(&mut self, transaction_id : u32, client_id : u16) {

        if let Some(transaction) = self.transactions.get(&transaction_id) {

            let (transaction_client_id, amount, _) = (transaction.0, transaction.1, transaction.2);

            if transaction_client_id == client_id {
                if let Some(account) = self.accounts.get_mut(&client_id) {
                    account.available_amount += amount;
                    account.held_amount -= amount;

                    //we resolved the transaction, so we can remove it from the map, since another future transaction cannot operate on a solved transaction
                    self.transactions.remove(&transaction_id);
                }
            }
        }
    }

    fn chargeback(&mut self, transaction_id : u32, client_id : u16) {

        if let Some(transaction) = self.transactions.get(&transaction_id) {

            let (transaction_client_id, amount, _) = (transaction.0, transaction.1, transaction.2);

            if transaction_client_id == client_id {
                if let Some(account) = self.accounts.get_mut(&client_id) {
                    account.total_amount -= amount;
                    account.held_amount -= amount;
                    account.locked = true;

                    //after chargeback, we remove the transaction, since no future transaction can operate on this transaction anymore
                    self.transactions.remove(&transaction_id);
                }
            }
        }
    }
    

    pub fn get_client_accounts(&mut self) -> Vec<ClientAccount> {
        self.accounts.values().cloned().collect()
    }

}