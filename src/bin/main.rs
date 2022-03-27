use payments_engine;

fn main() {
    match payments_engine::run() {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
