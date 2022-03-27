use std::fmt;
use std::io;

#[derive(Debug)]
pub enum PaymentError {
    NoFilePathProvided(String),
    AmountMissing(String),
}

pub fn from_no_file_provided(message: String) -> PaymentError {
    PaymentError::NoFilePathProvided(message)
}

pub fn from_amount_required(message: String) -> PaymentError {
    PaymentError::AmountMissing(message)
}

impl From<io::Error> for PaymentError {
    fn from(err: io::Error) -> PaymentError {
        PaymentError::NoFilePathProvided(format!("Cannot read file: {:?}", err))
    }
}

impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PaymentError::NoFilePathProvided(ref message) => write!(f, "{}", message),
            PaymentError::AmountMissing(ref message) => write!(f, "{}", message),
            // EnclaveError::Inner(ref message, ref inner_message) => {
            //     write!(f, "{}. Inner message: {}", message, inner_message)
            // }
            // EnclaveError::SgxOcall(ref sgx_status, ref message) => {
            //     write!(f, "SGX status: {}. Message: {}", sgx_status, message)
            // }
            // EnclaveError::AuthError(ref message) => {
            //     write!(f, "Authentication Message: {}", message)
            // }
        }
    }
}
