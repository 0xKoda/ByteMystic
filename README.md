## ByteMystic: Solidity bytecode-decoder

This Rust CLI application decodes a Solidity function call from a given bytecode input. It uses the ethabi crate to define the function signature and decode the function call from the bytecode. The decoded function call provides information about the function name and the input parameters.


# Getting started.

To run the application, use the cargo run command followed by the bytecode input:

`cargo run <bytecode>`


Replace `<bytecode>` with the actual bytecode you want to decode.

For example:
```
cargo run 6060604052341561000f57600080fd5b60d38061001d6000396000f300606060405260043610603f576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b11460445780636d4ce63c146062575b600080fd5b3415604e57600080fd5b606073565b005b3415606c57600080fd5b606272048080359060200190919050506078565b005b8060008190555050565b600080549050905600a165627a7a72305820d7f6cbc5e7003de6e1e5b76e8f0e8d5f3e1e7c81f4e07a0b5a1e5e7a5f4a9d6e0029
```


# Why 
This Rust CLI application is useful for developers working with Ethereum smart contracts, as it allows them to decode Solidity function calls from bytecode inputs. This can help in understanding the behavior of a contract for security purposes, debugging issues, or verifying that a contract is functioning as expected.

By using the `ethabi` crate, the application can easily handle various Solidity function signatures and decode the corresponding function calls from the provided bytecode. This makes it a valuable tool for developers working with Ethereum and Solidity.