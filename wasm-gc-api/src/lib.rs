extern crate parity_wasm;
#[macro_use]
extern crate log;
extern crate rustc_demangle;

mod gc;
mod error;
mod bitvec;

use std::any::Any;
use std::mem;
use std::path::Path;

use parity_wasm::elements::{
    Module,
    Serialize,
    Deserialize
};

pub use error::Error;

pub struct Config {
    demangle: bool,
    keep_debug: bool,
}

pub struct GcResult(Box<Module>);

impl Config {
    /// Creates a blank slate of configuration, ready to gc wasm files.
    pub fn new() -> Config {
        Config {
            demangle: true,
            keep_debug: false,
        }
    }

    /// Configures whether or not this will demangle symbols as part of the gc
    /// pass.
    pub fn demangle(&mut self, demangle: bool) -> &mut Self {
        self.demangle = demangle;
        self
    }

    /// Configures whether or not debug sections will be preserved.
    pub fn keep_debug(&mut self, keep_debug: bool) -> &mut Self {
        self.keep_debug = keep_debug;
        self
    }

    /// Runs gc passes over the wasm input module `input`, returning the
    /// serialized output.
    #[doc(hidden)] // deprecated, use `run` now.
    pub fn gc(&mut self, bytecode: &[u8]) -> Result<Vec<u8>, Error> {
        self.run(3, |_| bytecode.to_vec())?.into_bytes()
    }

    pub fn run<T: Any>(
        &mut self,
        mut module: T,
        into_bytes: impl FnOnce(T) -> Vec<u8>,
    ) -> Result<GcResult, Error> {
        if let Some(module) = (&mut module as &mut Any).downcast_mut() {
            self._gc(module);
            let module = mem::replace(module, Module::new(Vec::new()));
            return Ok(GcResult(Box::new(module)))
        }
        let bytecode = into_bytes(module);
        let mut module = Module::deserialize(&mut &bytecode[..])
            .map_err(error::from)?
            .parse_names()
            .map_err(|(mut l, _)| l.remove(0).1)
            .map_err(error::from)?;
        self._gc(&mut module);
        Ok(GcResult(Box::new(module)))
    }

    fn _gc(&mut self, module: &mut Module) {
        gc::run(self, module);
    }
}

impl GcResult {
    /// Attepts to downcast this `GcResult` into an instance of
    /// `parity_wasm::Module`.
    ///
    /// If your crate's `parity_wasm` crate is a different version than this
    /// crate then this method will fail and you'll need to use `into_bytes`.
    /// Otherwise the module is successfully extracted and returned.
    pub fn into_module<T: Any>(self) -> Result<T, Self> {
        let module = self.0 as Box<Any>;
        match module.downcast() {
            Ok(t) => Ok(*t),
            Err(box_any) => {
                match box_any.downcast::<Module>() {
                    Ok(box_module) => Err(GcResult(box_module)),
                    Err(_) => panic!(),
                }
            }
        }
    }

    /// Convert this `GcResult` into a serialized wasm module.
    ///
    /// Returns any error that happens during serialization, which shouldn't
    /// happen for valid modules.
    pub fn into_bytes(self) -> Result<Vec<u8>, Error> {
        let mut output = Vec::new();
        self.0.serialize(&mut output).map_err(error::from)?;
        Ok(output)
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
