use ethabi::{Function, Param, ParamType};
use std::env;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{} {}", "Usage:".red(), format!("{} <bytecode>", args[0]).red());
        return;
    }

    let bytecode = &args[1];

    // Define the Solidity function signature
    let function = Function {
        name: "exampleFunction".to_owned(),
        inputs: vec![
            Param {
                name: "input1".to_owned(),
                kind: ParamType::Uint(256),
                internal_type: None,
            },
            Param {
                name: "input2".to_owned(),
                kind: ParamType::Address,
                internal_type: None,
            },
        ],
        outputs: vec![],
        constant: false,
        state_mutability: ethabi::StateMutability::NonPayable,
    };

    // Decode the function call from bytecode
    match function.decode_input(&hex::decode(bytecode).unwrap()) {
        Ok(decoded) => println!("{} {:?}", "Decoded function call:".green(), decoded),
        Err(error) => eprintln!("{} {}", "Error decoding function call:".red(), error.to_string().red()),
    }
}
