mod decoder;
mod functions;

use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{} {}", "Usage:".red(), format!("{} [-e] <transaction_hash_or_bytecode>", args[0]).red());
        return;
    }

    let mut use_arbiscan = false;
    let input = if args[1] == "-e" {
        if args.len() != 3 {
            eprintln!("{} {}", "Usage:".red(), format!("{} -e <transaction_hash>", args[0]).red());
            return;
        }
        use_arbiscan = true;
        &args[2]
    } else {
        &args[1]
    };

    let bytecode = if use_arbiscan {
        fetch_bytecode_from_arbiscan(input)
    } else {
        Some(input.to_string())
    };

    if let Some(bytecode) = bytecode {
        println!("Bytecode: {}", bytecode);

        // Decode the function calls from bytecode
        for function in functions::functions() {
            match decoder::decode_function(&bytecode, &function) {
                Ok(decoded) => println!("{} {:?} {:?}", "Decoded function call:".green(), function.name, decoded),
                Err(error) => eprintln!("{} {} {}", "Error decoding function call:".red(), function.name, error.to_string().red()),
            }
        }
    } else {
        eprintln!("{} {}", "Error:".red(), "Failed to fetch or decode bytecode.".red());
    }
}

fn fetch_bytecode_from_arbiscan(transaction_hash: &str) -> Option<String> {
    let client = Client::new();

    // Fetch transaction data from Arbiscan (Blockscout API)
    let response = client
        .get(&format!(
            "https://api.arbiscan.io/api?module=proxy&action=eth_getTransactionByHash&txhash={}",
            transaction_hash
        ))
        .send();

        match response {
            Ok(mut resp) => {
                let json: Value = resp.json().unwrap();
                json["result"]["input"].as_str().map(|s| s.trim_start_matches("0x").to_string())
            }
            Err(error) => {
                eprintln!("{} {}", "Error fetching transaction data:".red(), error.to_string().red());
                None
            }
        }
    }