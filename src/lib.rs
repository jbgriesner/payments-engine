mod error;
mod models;
mod processor;
pub mod utils;

use csv::{ReaderBuilder, Trim};
use error::PaymentError;
pub use utils::get_first_arg;

#[allow(unused_mut)]
pub fn run(filepath: String, mut writer: impl std::io::Write) -> Result<(), PaymentError> {
    let file_buffer = utils::from_filepath(filepath)?;
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(file_buffer);

    let transactions_map = processor::from_reader(rdr);

    let mut wtr = csv::Writer::from_writer(writer);

    for (client_id, transactions) in transactions_map.iter() {
        let balance = processor::generate_client_balance(client_id, transactions);
        if let Ok(b) = balance {
            wtr.serialize(b).unwrap();
        }
    }

    wtr.flush().unwrap();
    Ok(())
}
