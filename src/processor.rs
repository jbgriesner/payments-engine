use crate::error::{from_amount_required, from_csv_parsing_error, PaymentError};
use crate::models::{Balance, Transaction, TxType};
use csv::Reader;
use std::collections::BTreeMap;
use std::io::BufReader;

/// Simple function that takes a reader on a csv file, and deserialize it into vectors of Transactions
/// These vectors are values in a resulting BTreeMap in which the client id are the keys
/// For this purpose I use a functional approach with a folding pattern
pub fn from_reader(
    mut rdr: Reader<BufReader<std::fs::File>>,
) -> Result<BTreeMap<u16, Vec<Transaction>>, PaymentError> {
    /// Given that the parsing could fail, I use the 'try_fold' method to handle a csv parsing error
    rdr.deserialize().try_fold(
        BTreeMap::new(),
        |mut hm, record: Result<Transaction, csv::Error>| {
            if let Ok(trans) = record {
                let trans_updated_amount = Transaction {
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
                Ok(hm)
            } else {
                Err(from_csv_parsing_error(
                    "An error occurred during the parsing of the file".to_string(),
                ))
            }
        },
    )
}

/// Main function that takes a client id and its associated vector of transactions, and generate 'balances'
/// Balances represent the status of the client account after having processed all the associated transactions
pub fn generate_client_balance(
    client_id: &u16,
    transactions: &Vec<Transaction>,
) -> Result<Balance, PaymentError> {
    let mut map_tx_amount = BTreeMap::new();
    let mut vec_tx_dispute = Vec::<u32>::new();

    let init_balance = Balance {
        client: *client_id,
        available: 0.0,
        held: 0.0,
        total: 0.0,
        locked: false,
    };

    transactions
        .iter()
        .try_fold(init_balance, |b, t| match t.tx_type {
            TxType::Deposit => {
                if let (Some(amount), false) = (t.amount, b.locked) {
                    map_tx_amount.insert(t.tx_id, amount);
                    Ok(Balance {
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
                if let (Some(amount), false) = (t.amount, b.locked) {
                    if amount > b.available {
                        Ok(b)
                    } else {
                        map_tx_amount.insert(t.tx_id, amount);
                        Ok(Balance {
                            total: b.total - amount,
                            available: b.available - amount,
                            ..b
                        })
                    }
                } else {
                    Err(from_amount_required(
                        "An amount is required for a withdrawal, but is missing".to_string(),
                    ))
                }
            }
            TxType::Dispute => {
                if let (Some(disputed_amount), false) = (map_tx_amount.get(&t.tx_id), b.locked) {
                    vec_tx_dispute.push(t.tx_id);
                    Ok(Balance {
                        held: b.held + disputed_amount,
                        available: b.available - disputed_amount,
                        ..b
                    })
                } else {
                    Ok(b)
                }
            }
            TxType::Resolve => {
                if let (Some(disputed_amount), true, false) = (
                    map_tx_amount.get(&t.tx_id),
                    vec_tx_dispute.contains(&t.tx_id),
                    b.locked,
                ) {
                    vec_tx_dispute.retain(|v| *v != t.tx_id);
                    Ok(Balance {
                        held: b.held - disputed_amount,
                        available: b.available + disputed_amount,
                        ..b
                    })
                } else {
                    Ok(b)
                }
            }
            TxType::Chargeback => {
                if let (Some(disputed_amount), true) = (
                    map_tx_amount.get(&t.tx_id),
                    vec_tx_dispute.contains(&t.tx_id),
                ) {
                    Ok(Balance {
                        held: b.held - disputed_amount,
                        total: b.total - disputed_amount,
                        locked: true,
                        ..b
                    })
                } else {
                    Ok(b)
                }
            }
        })
}
