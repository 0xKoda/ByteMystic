mod decoder;
mod functions;

use std::env;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{} {}", "Usage:".red(), format!("{} <bytecode>", args[0]).red());
        return;
    }

    let bytecode = &args[1];

    // Decode the function calls from bytecode
    for function in functions::functions() {
        match decoder::decode_function(bytecode, &function) {
            Ok(decoded) => println!("{} {:?} {:?}", "Decoded function call:".green(), function.name, decoded),
            Err(error) => eprintln!("{} {} {}", "Error decoding function call:".red(), function.name, error.to_string().red()),
        }
    }
}
