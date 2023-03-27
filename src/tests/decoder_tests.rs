use ByteMystic::decoder;
use ByteMystic::functions;
use ByteMystic::ethabi::{Function, Param, ParamType};

fn example_functions() -> Vec<Function> {
    functions::functions()
}

fn test_bytecode() -> &'static str {
    "6060604052341561000f57600080fd5b60d38061001d6000396000f300606060405260043610603f576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff168063771602f7146044575b600080fd5b3415604e57600080fd5b606260048080359060200190919050506078565b6040518082815260200191505060405180910390f35b60008160020290509190505600a165627a7a72305820e7f6d5b7e0f1f8b619728470f5d847f6e"
}

#[test]
fn test_decode_function() {
    let bytecode = test_bytecode();
    let functions = example_functions();

    for function in functions {
        let result = decoder::decode_function(&bytecode, &function);
        match result {
            Ok(decoded) => {
                if function.name == "add" {
                    assert_eq!(decoded.name, "add");
                    assert_eq!(decoded.inputs.len(), 2);
                    assert_eq!(decoded.outputs.len(), 1);
                }
            }
            Err(_) => {
                
            }
        }
    }
}
