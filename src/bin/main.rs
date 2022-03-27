use payments_engine;

fn main() {
    if let Ok(filepath) = payments_engine::get_first_arg() {
        match payments_engine::run(filepath, &mut std::io::stdout()) {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
