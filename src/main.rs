extern crate env_logger;
extern crate wasm_gc;
extern crate getopts;

use std::env;
use std::fs::File;
use std::io::{Read, Write};

use getopts::Options;

fn main() {
    env_logger::init();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("", "no-demangle", "don't demangle symbol names");
    opts.optflag("h", "help", "print this help menu");
	let args: Vec<_> = env::args().collect();
	let program = args[0].clone();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        return print_usage(&program, opts)
    }
    let (input, output) = match matches.free.len() {
        0 => return print_usage(&program, opts),
        1 => {
            let input = matches.free[0].clone();
            match matches.opt_str("o"){
                None => (input.clone(), input),
                Some(s) => (input, s),
            }
        }
        2 => (matches.free[0].clone(), matches.free[1].clone()),
        _ => return print_usage(&program, opts),
    };

    let mut contents = Vec::new();
    File::open(&input).unwrap().read_to_end(&mut contents).unwrap();

    let mut cfg = wasm_gc::Config::new();
    cfg.demangle(!matches.opt_present("no-demangle"));
    let result = cfg.gc(&contents).expect("failed to parse wasm module");
    File::create(&output).unwrap().write_all(&result).unwrap();
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] <INPUT> [OUTPUT]", program);
    print!("{}", opts.usage(&brief));
    println!("
A postprocessing command for wasm files to garbage-collect unused
imports, internal functions, types, etc. This is intended to be
similar to the functionality provided by `--gc-sections` by linkers
in that it is not intended to modify the functionality of the wasm
binary, only make it a little smaller.

Usage of this command typically looks like:

    # Read and write output to one file
    wasm-gc foo.wasm

    # Read input from one file and write it to another file
    wasm-gc input.wasm output.wasm

    # Passing various options
    wasm-gc --no-demangle input.wasm -o output.wasm

Please reports bugs to https://github.com/alexcrichton/wasm-gc if you find
them!
");
}
