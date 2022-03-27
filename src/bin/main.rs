use payments_engine;

fn main() {
    // I get the first arg of the CLI (i.e. the filepath of the csv) here in the main in order to be able to call
    // integration tests with different filepaths
    if let Ok(filepath) = payments_engine::get_first_arg() {
        match payments_engine::run(filepath, &mut std::io::stdout()) {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
