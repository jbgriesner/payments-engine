## Get Started

### Requirements:
- the stable `rustup` toolchain, and `cargo` in the PATH.

### Launch the app
To run the app, just execute from the root folder of the project the following command:
```shell
cargo run -- <file_path>
```
Examples of possible input test files are provided under `tests/features/` folder.

### Important Assumption
We assume that a `Withdrawal` operation can be disputed. It is not perfectly clear if, when a past withdrawal transaction is disputed, the available amount should be increased back or decreased, given that it could make more sense to increase it, assuming that the withdraw failed of something like that. However without more specification I cannot be sure of that. For this reason I assume here that, when a withdrawal operation is disputed, the corresponding amount is substracted from the available funds.

## Design
First of all, I parse the input transactions file with a `BufReader`, and I put them in a `csv::Reader`. To allow any number of whitespaces/tabs between fields, I use the `csv::Trim` enum. I use the Rust Type System to be sure I are parsing correctly the fields. All structs & enums required for this are defined under the `models` folder.
Then I follow a 2 phases approach to solve the problem:
- first I build a transactions map: it associates each client id with the vector of all his/her corresponding transactions. This transactions map is actually a `BTreeMap` for sake of determinism of the integration tests, given that `BTreeMap` allows us to iterate entries in the same order of the keys. 
- then in the second phase I process the transactions for each client by handling each transaction type in order to generate the balance of each user. Once the user's balance has been generated, it is pushed into a `Writer`, which is `std::io::stdout()` for the binary executable, and a vector of bytes for the integration tests. 
All this logic is in the `processor` module of the project.
To handle any kind of error that could happen, I define a custom struct in the `error` module. The project should never "panic".

## Possible Improvements
- regarding the function `generate_client_balance()`, I could move each processing of a specific type of transaction into a dedicated service in a dedicated module in order to increase separation of concerns in the code and reduce coupling.
- to allow to reuse this backend implementation in a distributed environment, I could make processing function async for instance, in order to be able to stream online all incoming data without having to put them all in memory.
- some unit tests could have been added as well. However, given that the project logic is so simple and context dependent, integration tests only are much more useful here
- more comments could also be added. However, for the same reason (simple logic), I didn't put a lot.

## Tests

Integration tests are provided under `tests/` folder. To run them, simply execute from the root folder of the project the following command:

```shell
cargo test
```

## Summary
Some questions have been asked. Here are below some brief answers with more or less details:

|Feature       | Result      | 
|---           |:-:          |
| Does the app build?             |    Yes         |  
| Does it read and write data?             |   Yes          |         
| Is it properly formatted?             |     Yes        |         
| Do you handle all of the cases?           |    Yes         |  
| are you handling them correctly            |    Yes         |  
| How do you know this           |    thanks to the tests + Rust Type System        |  
| Did you test against sample data?          |    Yes         |  
| Did you write unit tests for the complicated bits?            |    No         |  
| Or are you using the type system to ensure correctness?            |    Yes         |  
| Are you doing something dangerous?            |    No, all errors are handled         |  
| How are you handling errors?         |    with specific Error type         |  
| Can you stream values through memory as opposed to loading the entire data set upfront?        |    Yes it's possible easily      |  
| in a server, and these CSVs came from thousands of concurrent TCP streams       |    simply with a thread pool, or an async runtime after making all functions async       |  


