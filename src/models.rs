use serde::{Deserialize, Serialize};
use std::fmt;

/// An enum type to represent the possible types of a transaction
#[derive(Deserialize, Debug)]
pub enum TxType {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    Chargeback,
}

/// We wan to be able to display a transaction type (for debug mainly)
impl fmt::Display for TxType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TxType::Deposit => write!(f, "deposit"),
            TxType::Withdrawal => write!(f, "withdrawal"),
            TxType::Dispute => write!(f, "dispute"),
            TxType::Resolve => write!(f, "resolve"),
            TxType::Chargeback => write!(f, "chargeback"),
        }
    }
}

/// A struct type used by Serde for parsing input csv files
#[derive(Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: TxType,
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub tx_id: u32,
    pub amount: Option<f64>,
}

/// A struct type to write a csv on the standard output
#[derive(Serialize, Debug)]
pub struct Balance {
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}
