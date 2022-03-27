use payments_engine;

static FILEPATH_DEPOSITS_AND_WITHDRAW: &str = &"./tests/features/input.data";
static FILEPATH_DISPUTE: &str = &"./tests/features/input_with_dispute.data";
static FILEPATH_RESOLVE: &str = &"./tests/features/input_with_resolve.data";
static FILEPATH_CHARGEBACK: &str = &"./tests/features/input_with_chargeback.data";

#[test]
fn test_deposits_and_withdraw() {
    let mut result = Vec::new();

    // #[allow(unused_mut)]
    payments_engine::run(FILEPATH_DEPOSITS_AND_WITHDRAW.to_string(), &mut result).unwrap();

    let valid_output =
        "client,available,held,total,locked\n1,1.5,0.0,1.5,false\n2,1.0,0.0,1.0,false\n"
            .to_string();
    let generated_output = String::from_utf8(result).unwrap();

    assert_eq!(generated_output, valid_output);
}

#[test]
fn test_dispute() {
    let mut result = Vec::new();

    // #[allow(unused_mut)]
    payments_engine::run(FILEPATH_DISPUTE.to_string(), &mut result).unwrap();

    let valid_output = "client,available,held,total,locked\n1,100.0,10.0,110.0,false\n".to_string();
    let generated_output = String::from_utf8(result).unwrap();

    assert_eq!(generated_output, valid_output);
}

#[test]
fn test_resolve() {
    let mut result = Vec::new();

    // #[allow(unused_mut)]
    payments_engine::run(FILEPATH_RESOLVE.to_string(), &mut result).unwrap();

    let valid_output = "client,available,held,total,locked\n1,100.0,10.0,110.0,false\n".to_string();
    let generated_output = String::from_utf8(result).unwrap();

    assert_eq!(generated_output, valid_output);
}

#[test]
fn test_chargeback() {
    let mut result = Vec::new();

    // #[allow(unused_mut)]
    payments_engine::run(FILEPATH_CHARGEBACK.to_string(), &mut result).unwrap();

    let valid_output = "client,available,held,total,locked\n1,100.0,10.0,110.0,true\n".to_string();
    let generated_output = String::from_utf8(result).unwrap();

    assert_eq!(generated_output, valid_output);
}
