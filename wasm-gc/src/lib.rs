extern crate parity_wasm;
#[macro_use]
extern crate log;
extern crate rustc_demangle;

mod gc;
mod error;

use std::path::Path;
use parity_wasm::elements::{
    Module,
    Serialize,
    Deserialize
};

use gc::garbage_collect;
pub use error::Error;

/// Garbage collects the webassembly bytecode from `input_path` and saves it to `output_path`.
pub fn garbage_collect_file<I, O>(input_path: I, output_path: O) -> Result<(), Error>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let mut module = parity_wasm::deserialize_file(input_path.as_ref())?;
    garbage_collect(&mut module);
    parity_wasm::serialize_to_file(output_path.as_ref(), module)?;

    Ok(())
}

/// Garbage collects given webassembly bytecode.
pub fn garbage_collect_slice(mut bytecode: &[u8]) -> Result<Vec<u8>, Error> {
    let mut module = Module::deserialize(&mut bytecode)?;
    garbage_collect(&mut module);

    let mut output = Vec::new();
    module.serialize(&mut output)?;

    Ok(output)
}
