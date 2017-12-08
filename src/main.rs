extern crate env_logger;
extern crate wasm_gc;

use std::env;

fn main() {
    env_logger::init().unwrap();

    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("Usage: {} input.wasm output.wasm", args[0]);
        return
    }

    wasm_gc::garbage_collect_file(&args[1], &args[2]).unwrap();
}
