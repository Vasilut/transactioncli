use std::{env, error::Error};

pub mod entities;
pub mod service;
use entities::{Transaction, ClientAccount};
use service::PaymentEngine;

fn main() -> Result<(), Box<dyn Error>>{

    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <transactions.csv>");
        return Err("Invalid number of arguments".into());
    }

    let input_file = &args[1];
    let mut file_reader = csv::Reader::from_path(input_file)?;

    let mut payment_engine = PaymentEngine::new();

    for records in file_reader.deserialize::<Transaction>() {
        match records {
            Ok(current_transaction) => {
                println!("{:?}", current_transaction);
            }
            Err(e) => {
                eprintln!("Error processing record: {}", e);
                continue; 
            }
        }
    }

    Ok(())
}
