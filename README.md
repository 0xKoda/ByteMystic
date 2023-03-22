## ByteMystic: Solidity bytecode-decoder

This Rust CLI application decodes a Solidity function call from a given bytecode input. It uses the ethabi crate to define the function signature and decode the function call from the bytecode. The decoded function call provides information about the function name and the input parameters.


# Getting started.

To run the application, use the cargo run command followed by the bytecode input:

`cargo run <bytecode>`


Replace `<bytecode>` with the actual bytecode you want to decode.

For example:
```
cargo run 1234567890123456789012345678901234567890123456789012345678901234000000000000000000000000000000000000000000000000000000000000000001

```



# Why 
This Rust CLI application is useful for developers working with Ethereum smart contracts, as it allows them to decode Solidity function calls from bytecode inputs. This can help in understanding the behavior of a contract for security purposes, debugging issues, or verifying that a contract is functioning as expected.

By using the `ethabi` crate, the application can easily handle various Solidity function signatures and decode the corresponding function calls from the provided bytecode. This makes it a valuable tool for developers working with Ethereum and Solidity.