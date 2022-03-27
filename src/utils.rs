use crate::error::{from_no_file_provided, PaymentError};
use std::env;
use std::fs::File;
use std::io;
use std::path::Path;

pub fn get_first_arg() -> Result<String, PaymentError> {
    match env::args().nth(1) {
        None => Err(from_no_file_provided(
            "expected 1 argument, but got none".to_string(),
        )),
        Some(file_path) => Ok(file_path),
    }
}

pub fn from_filepath<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
