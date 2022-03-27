use payments_engine;
use std::env;

#[test]
fn test_deposits_and_withdraw() {
    let filepath = "./tests/input.data".to_string();
    let mut result = Vec::new();

    // #[allow(unused_mut)]
    payments_engine::run(filepath, &mut result).unwrap();

    let valid_output =
        "client,available,held,total,locked\n1,1.5,0.0,1.5,0.0\n2,1.0,0.0,1.0,0.0\n".to_string();
    let generated_output = String::from_utf8(result).unwrap();

    assert_eq!(generated_output, valid_output);
}

// #[test]
// fn test_dispute() {
//     assert_eq!(5, 5);
// }

// #[test]
// fn test_resolve() {
//     assert_eq!(5, 5);
// }

// #[test]
// fn test_chargeback() {
//     assert_eq!(5, 5);
// }
