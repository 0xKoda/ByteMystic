## ByteMystic: Solidity bytecode-decoder

This Rust CLI application decodes a Solidity function call from a given bytecode input or from a given tx hash on arbitrum. It uses the ethabi crate to define the function signature and decode the function call from the bytecode. The decoded function call provides information about the function name and the input parameters.


# Getting started.

To run the application, use the cargo run command followed by the bytecode input:

`cargo run <bytecode>`


Replace `<bytecode>` with the actual bytecode you want to decode.


or to pull the bytecode from arbiscan, use the `-- -e` flag with the tx hash:

```sh
cargo run -- -e 0x234bf2f03473af0318fa9cea1d2f883bd9f7b861864c6c8409d81d72856becbc
```


# Why 
This Rust CLI application is useful for developers working with Ethereum smart contracts, as it allows them to decode Solidity function calls from bytecode inputs. This can help in understanding the behavior of a contract for security purposes, debugging issues, or verifying that a contract is functioning as expected.
