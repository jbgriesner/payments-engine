## Get Started

### Summary

|Feature       | Result      | Comment      |
|---           |:-:          |:-:           |
| Does the app build?             |    Yes         |              |
| Does it read and write data?             |   Yes          |              |
| Is it properly formatted?             |     Yes        |              |

### Requirements:
- the stable `rustup` toolchain, and cargo in the PATH.

### Launch the app
To run the app, just execute from the root folder of the project the following command:
```shell
cargo run -- <file_path>
```
Examples of possible input test files are provided under `tests/features/` folder.

## Design
First of all, we parse the input transactions file with a `BufReader`, and we put them in a `csv::Reader`. To allow any number of whitespaces/tabs between fields, we use the `csv::Trim` enum. We use the Rust Type System to be sure we are parsing correctly the fields. All structs & enums required for this are defined under the `models` folder.
Then we follow a 2 phases approach to solve the problem:
- first we build a transactions map: it associates each client id with the vector of all his/her corresponding transactions. This transactions map is actually a `BTreeMap` for sake of determinism of the integration tests, given that `BTreeMap` allows us to iterate entries in the same order of the keys. 
- then in the second phase we process the transactions for each client by handling each transaction type in order to generate the balance of each user. Once the user's balance has been generated, it is pushed into a `Writer`, which is `std::io::stdout()` for the binary executable, and a vector of bytes for the integration tests. 
All this logic is in the `processor` module of the project.
To handle any kind of error that could happen, we define a custom struct in the `error` module. The project should never "panic".

## Possible Improvements
- regarding the function `generate_client_balance()`, we could move each processing of a specific type of transaction into a dedicated service in a dedicated module in order to increase separation of concerns in the code and reduce coupling.
- 
## Tests

Integration tests are provided under `tests/` folder. To run them, simply execute from the root folder of the project the following command:

```shell
cargo test
```