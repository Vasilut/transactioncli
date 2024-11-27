
use std::collections::HashMap;

use crate::entities::{ClientAccount, Transaction};

pub struct PaymentEngine {
    accounts: HashMap<u16, ClientAccount>, 
    transactions: HashMap<u32, (u16, f64, bool)>, //(client_id, amount, is_disputted)
}

impl PaymentEngine {
    pub fn new() -> Self {
        PaymentEngine {
            accounts : HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    fn process_transaction(&mut self, transaction : Transaction) {

    }

    fn deposit(&mut self, transaction_id : u32, client_id : u16, amount: f64) {

    }

    fn withdraw(&mut self, transaction_id : u32, client_id : u16, amount: f64) {

    } 

    fn dispute(&mut self, transaction_id : u32, client_id : u16) {

    }

    fn resolve(&mut self, transaction_id : u32, client_id : u16) {

    }

    fn chargeback(&mut self, transaction_id : u32, client_id : u16) {

    }

}