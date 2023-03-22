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
            name: "exampleFunction2".to_owned(),
            inputs: vec![
                Param {
                    name: "input1".to_owned(),
                    kind: ParamType::FixedBytes(32),
                    internal_type: None,
                },
                Param {
                    name: "input2".to_owned(),
                    kind: ParamType::Bool,
                    internal_type: None,
                },
            ],
            outputs: vec![],
            constant: false,
            state_mutability: ethabi::StateMutability::NonPayable,
        },
        // Add more functions here
    ]
}
