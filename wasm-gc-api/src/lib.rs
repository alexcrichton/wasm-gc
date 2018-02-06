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

pub use error::Error;

pub struct Config {
    demangle: bool,
}

impl Config {
    /// Creates a blank slate of configuration, ready to gc wasm files.
    pub fn new() -> Config {
        Config {
            demangle: true,
        }
    }

    /// Configures whether or not this will demangle symbols as part of the gc
    /// pass.
    pub fn demangle(&mut self, demangle: bool) -> &mut Self {
        self.demangle = demangle;
        self
    }

    /// Runs gc passes over the wasm input module `input`, returning the
    /// serialized output.
    pub fn gc(&mut self, mut bytecode: &[u8]) -> Result<Vec<u8>, Error> {
        let mut module = Module::deserialize(&mut bytecode)
            .map_err(error::from)?
            .parse_names()
            .map_err(|(mut l, _)| l.remove(0).1)
            .map_err(error::from)?;
        self._gc(&mut module);
        let mut output = Vec::new();
        module.serialize(&mut output).map_err(error::from)?;
        Ok(output)
    }

    fn _gc(&mut self, module: &mut Module) {
        gc::run(self, module);
    }
}

/// Garbage collects the webassembly bytecode from `input_path` and saves it to `output_path`.
pub fn garbage_collect_file<I, O>(input_path: I, output_path: O) -> Result<(), Error>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    _gc_file(input_path.as_ref(), output_path.as_ref())
}

fn _gc_file(input: &Path, output: &Path) -> Result<(), Error> {
    let mut module = parity_wasm::deserialize_file(input)
        .map_err(error::from)?
        .parse_names()
        .map_err(|(mut l, _)| l.remove(0).1)
        .map_err(error::from)?;
    Config::new()._gc(&mut module);
    parity_wasm::serialize_to_file(output, module).map_err(error::from)?;

    Ok(())
}

/// Garbage collects given webassembly bytecode.
pub fn garbage_collect_slice(bytecode: &[u8]) -> Result<Vec<u8>, Error> {
    Config::new().gc(bytecode)
}
