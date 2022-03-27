use crate::error::{from_amount_required, PaymentError};
use crate::models::{Balance, Transaction, TxType};
use csv::Reader;
use std::collections::BTreeMap;
use std::io::BufReader;

pub fn from_reader(mut rdr: Reader<BufReader<std::fs::File>>) -> BTreeMap<u16, Vec<Transaction>> {
    rdr.deserialize().fold(BTreeMap::new(), |mut hm, record| {
        let trans: Transaction = record.unwrap();
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
        hm
    })
}

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
