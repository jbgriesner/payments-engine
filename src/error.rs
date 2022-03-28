use std::fmt;
use std::io;

/// A struct to encapsulate all possible errors that could happen in the execution of the process
#[derive(Debug)]
pub enum PaymentError {
    NoFilePathProvided(String),
    AmountMissing(String),
    CsvError(String),
}

impl From<io::Error> for PaymentError {
    fn from(err: io::Error) -> PaymentError {
        PaymentError::NoFilePathProvided(format!("Cannot read file: {:?}", err))
    }
}

impl From<csv::Error> for PaymentError {
    fn from(err: csv::Error) -> PaymentError {
        PaymentError::CsvError(format!("csv Error: {:?}", err))
    }
}

pub fn from_amount_required(message: String) -> PaymentError {
    PaymentError::AmountMissing(message)
}

pub fn from_no_file_provided(message: String) -> PaymentError {
    PaymentError::NoFilePathProvided(message)
}

pub fn from_csv_parsing_error(message: String) -> PaymentError {
    PaymentError::CsvError(message)
}

impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PaymentError::NoFilePathProvided(ref message) => write!(f, "{}", message),
            PaymentError::AmountMissing(ref message) => write!(f, "{}", message),
            PaymentError::CsvError(ref message) => write!(f, "{}", message),
        }
    }
}
