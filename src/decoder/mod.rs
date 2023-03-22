use ethabi::Function;

pub fn decode_function(bytecode: &str, function: &Function) -> Result<Vec<ethabi::Token>, ethabi::Error> {
    function.decode_input(&hex::decode(bytecode)?)
}
