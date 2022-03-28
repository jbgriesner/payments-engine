use crate::error::from_no_file_provided;
use crate::error::PaymentError;
use csv::Reader;
use csv::{ReaderBuilder, Trim};
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;

/// This module contains only simple functions

pub fn reader_from_filepath(
    filepath: String,
) -> Result<Reader<BufReader<std::fs::File>>, PaymentError> {
    let file_buffer = from_filepath(filepath)?;
    let rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(file_buffer);
    Ok(rdr)
}

pub fn get_first_arg() -> Result<String, PaymentError> {
    match env::args().nth(1) {
        None => Err(from_no_file_provided(
            "expected 1 argument, but got none".to_string(),
        )),
        Some(file_path) => Ok(file_path),
    }
}

fn from_filepath<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
