mod error;
mod models;
mod processor;
pub mod utils;

use error::PaymentError;
pub use utils::get_first_arg;

/// The main function of the project that is called by the ./bin/main.rs
/// A mutable 'writer' is passed as an argument to be able to push on different output (standard output or an array of bytes for instance, which is useful for tests)
#[allow(unused_mut)]
pub fn run(filepath: String, mut writer: impl std::io::Write) -> Result<(), PaymentError> {
    let mut rdr = utils::reader_from_filepath(filepath)?;

    let transactions_map = processor::from_reader(rdr)?;

    let mut wtr = csv::Writer::from_writer(writer);

    for (client_id, transactions) in transactions_map.iter() {
        let balance = processor::generate_client_balance(client_id, transactions)?;

        wtr.serialize(balance)?;
    }

    wtr.flush()?;
    Ok(())
}
