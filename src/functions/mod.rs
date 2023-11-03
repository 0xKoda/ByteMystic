use ethabi::{Function, Param, ParamType};

pub fn functions() -> Vec<Function> {
    vec![
        Function {
            name: "exampleFunction1".to_owned(),
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
        },
        Function {
            name: "transfer".to_owned(),
            inputs: vec![
                Param {
                    name: "to".to_owned(),
                    kind: ParamType::Address,
                    internal_type: None,
                },
                Param {
                    name: "value".to_owned(),
                    kind: ParamType::Uint(256),
                    internal_type: None,
                },
            ],
            outputs: vec![Param {
                name: "success".to_owned(),
                kind: ParamType::Bool,
                internal_type: None,
            }],
            constant: false,
            state_mutability: ethabi::StateMutability::NonPayable,
        },
        Function {
            name: "approve".to_owned(),
            inputs: vec![
                Param {
                    name: "spender".to_owned(),
                    kind: ParamType::Address,
                    internal_type: None,
                },
                Param {
                    name: "value".to_owned(),
                    kind: ParamType::Uint(256),
                    internal_type: None,
                },
            ],
            outputs: vec![Param {
                name: "success".to_owned(),
                kind: ParamType::Bool,
                internal_type: None,
            }],
            constant: false,
            state_mutability: ethabi::StateMutability::NonPayable,
        },
        Function {
            name: "approve".to_owned(),
            inputs: vec![
                Param {
                    name: "spender".to_owned(),
                    kind: ParamType::Address,
                    internal_type: None,
                },
                Param {
                    name: "value".to_owned(),
                    kind: ParamType::Uint(256),
                    internal_type: None,
                },
            ],
            outputs: vec![Param {
                name: "success".to_owned(),
                kind: ParamType::Bool,
                internal_type: None,
            }],
            constant: false,
            state_mutability: ethabi::StateMutability::NonPayable,
        },
        Function {
            name: "transferFrom".to_owned(),
            inputs: vec![
                Param {
                    name: "from".to_owned(),
                    kind: ParamType::Address,
                    internal_type: None,
                },
                Param {
                    name: "to".to_owned(),
                    kind: ParamType::Address,
                    internal_type: None,
                },
                Param {
                    name: "tokenId".to_owned(),
                    kind: ParamType::Uint(256),
                    internal_type: None,
                },
            ],
            outputs: vec![],
            constant: false,
            state_mutability: ethabi::StateMutability::NonPayable,
        },
        Function {
            name: "add".to_owned(),
            inputs: vec![
                Param {
                    name: "a".to_owned(),
                    kind: ParamType::Uint(256),
                    internal_type: None,
                },
                Param {
                    name: "b".to_owned(),
                    kind: ParamType::Uint(256),
                    internal_type: None,
                },
            ],
            outputs: vec![
                Param {
                    name: "sum".to_owned(),
                    kind: ParamType::Uint(256),
                    internal_type: None,
                },
            ],
            constant: false,
            state_mutability: ethabi::StateMutability::NonPayable,
        },
        Function {
            name: "swapandbridge".to_owned(),
            inputs: vec![
                Param {
                    name: "_weth".to_owned(),
                    kind: ParamType::Address,
                    internal_type: None,
                },
                Param {
                    name: "_oft".to_owned(),
                    kind: ParamType::Address,
                    internal_type: None,
                },
                Param {
                    name: "_swapRouter".to_owned(),
                    kind: ParamType::Address,
                    internal_type: None,
                },
            ],
            outputs: vec![],
            constant: false,
            state_mutability: ethabi::StateMutability::NonPayable,
        },

                
        
    ]
}
