mod error;
mod models;
mod utils;

use csv::ReaderBuilder;
use csv::Trim;
use error::{from_amount_required, PaymentError};
use models::Transaction;
use models::TxType;
use std::collections::HashMap;
use std::io;

pub fn run() -> Result<(), PaymentError> {
    let filepath = utils::get_first_arg()?;

    if let Ok(file_buffer) = utils::from_filepath(filepath) {
        let mut rdr = ReaderBuilder::new()
            .trim(Trim::All)
            .from_reader(file_buffer);

        let transactions_map = rdr.deserialize().fold(HashMap::new(), |mut hm, record| {
            let trans: Transaction = record.unwrap();
            let trans_updated_amount = models::Transaction {
                amount: if let Some(amount) = trans.amount {
                    Some(f64::trunc(amount * 10000.0) / 10000.0)
                } else {
                    None
                },
                ..trans
            };

            hm.entry(trans.client_id)
                .or_insert(Vec::new())
                .push(trans_updated_amount);
            hm
        });

        let mut wtr = csv::Writer::from_writer(io::stdout());

        for (client_id, transactions) in transactions_map.iter() {
            let mut map_tx_amount = HashMap::new();

            let init_balance = models::Balance {
                client: *client_id,
                available: 0.0,
                held: 0.0,
                total: 0.0,
                locked: 0.0,
            };

            let balance = transactions
                .iter()
                .try_fold(init_balance, |b, t| match t.tx_type {
                    TxType::Deposit => {
                        if let Some(amount) = t.amount {
                            map_tx_amount.insert(t.tx_id, amount);
                            Ok(models::Balance {
                                total: b.total + amount,
                                available: b.available + amount,
                                ..b
                            })
                        } else {
                            Err(from_amount_required(
                                "An amount is required for a deposit, but is missing".to_string(),
                            ))
                        }
                    }
                    TxType::Withdrawal => {
                        if let Some(amount) = t.amount {
                            if amount > b.available {
                                Ok(b)
                            } else {
                                map_tx_amount.insert(t.tx_id, amount);
                                Ok(models::Balance {
                                    total: b.total - amount,
                                    available: b.available - amount,
                                    ..b
                                })
                            }
                        } else {
                            Err(from_amount_required(
                                "An amount is required for a withdrawal, but is missing"
                                    .to_string(),
                            ))
                        }
                    }
                    TxType::Dispute => {
                        if let Some(disputed_amount) = map_tx_amount.get(&t.tx_id) {
                            Ok(models::Balance {
                                held: b.held + disputed_amount,
                                available: b.available - disputed_amount,
                                ..b
                            })
                        } else {
                            Ok(b)
                        }
                    }
                    TxType::Resolve => {
                        if let Some(disputed_amount) = map_tx_amount.get(&t.tx_id) {
                            Ok(models::Balance {
                                held: b.held - disputed_amount,
                                available: b.available + disputed_amount,
                                ..b
                            })
                        } else {
                            Ok(b)
                        }
                    }
                    TxType::Chargeback => {
                        if let Some(disputed_amount) = map_tx_amount.get(&t.tx_id) {
                            Ok(models::Balance {
                                held: b.held - disputed_amount,
                                total: b.total - disputed_amount,
                                ..b
                            })
                        } else {
                            Ok(b)
                        }
                    }
                });
            if let Ok(b) = balance {
                wtr.serialize(b).unwrap();
            }
        }

        wtr.flush().unwrap();
    }

    Ok(())
}
