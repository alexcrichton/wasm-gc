use std::error;
use std::fmt;
use parity_wasm::elements::Error as ParityWasmError;

/// The error type for garbage collecting webassembly bytecode.
#[derive(Debug)]
pub struct Error(ParityWasmError);

impl error::Error for Error {
    fn description(&self) -> &str {
        "webassembly garbage collection failed"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub fn from(parity: ParityWasmError) -> Error {
    Error(parity)
}
